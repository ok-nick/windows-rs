#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AnimationDirection(pub i32);
impl AnimationDirection {
    pub const Left: AnimationDirection = AnimationDirection(0i32);
    pub const Top: AnimationDirection = AnimationDirection(1i32);
    pub const Right: AnimationDirection = AnimationDirection(2i32);
    pub const Bottom: AnimationDirection = AnimationDirection(3i32);
}
impl ::core::convert::From<i32> for AnimationDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AnimationDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AnimationDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.AnimationDirection;i4)");
}
impl ::windows::core::DefaultType for AnimationDirection {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBarButtonTemplateSettings(pub ::windows::core::IInspectable);
impl AppBarButtonTemplateSettings {
    pub fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBarButtonTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarButtonTemplateSettings;{cbc9b39d-0c95-4951-bff2-13963691c366})");
}
unsafe impl ::windows::core::Interface for AppBarButtonTemplateSettings {
    type Vtable = IAppBarButtonTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbc9b39d_0c95_4951_bff2_13963691c366);
}
impl ::windows::core::RuntimeName for AppBarButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarButtonTemplateSettings";
}
impl ::core::convert::From<AppBarButtonTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: AppBarButtonTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBarButtonTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &AppBarButtonTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBarButtonTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: AppBarButtonTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBarButtonTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &AppBarButtonTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppBarButtonTemplateSettings> for super::super::DependencyObject {
    fn from(value: AppBarButtonTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&AppBarButtonTemplateSettings> for super::super::DependencyObject {
    fn from(value: &AppBarButtonTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for AppBarButtonTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarButtonTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBarTemplateSettings(pub ::windows::core::IInspectable);
impl AppBarTemplateSettings {
    #[cfg(feature = "Foundation")]
    pub fn ClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CompactVerticalDelta(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CompactRootMargin(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn MinimalVerticalDelta(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn MinimalRootMargin(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn HiddenVerticalDelta(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn HiddenRootMargin(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn NegativeCompactVerticalDelta(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAppBarTemplateSettings2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeMinimalVerticalDelta(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAppBarTemplateSettings2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeHiddenVerticalDelta(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAppBarTemplateSettings2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBarTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarTemplateSettings;{bcc2a863-eb35-423c-8389-d7827be3bf67})");
}
unsafe impl ::windows::core::Interface for AppBarTemplateSettings {
    type Vtable = IAppBarTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcc2a863_eb35_423c_8389_d7827be3bf67);
}
impl ::windows::core::RuntimeName for AppBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarTemplateSettings";
}
impl ::core::convert::From<AppBarTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: AppBarTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBarTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &AppBarTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBarTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: AppBarTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBarTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &AppBarTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: AppBarTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&AppBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: &AppBarTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for AppBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &AppBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for AppBarTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBarToggleButtonTemplateSettings(pub ::windows::core::IInspectable);
impl AppBarToggleButtonTemplateSettings {
    pub fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBarToggleButtonTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarToggleButtonTemplateSettings;{aaf99c48-d8f4-40d9-9fa3-3a64f0fec5d8})");
}
unsafe impl ::windows::core::Interface for AppBarToggleButtonTemplateSettings {
    type Vtable = IAppBarToggleButtonTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaf99c48_d8f4_40d9_9fa3_3a64f0fec5d8);
}
impl ::windows::core::RuntimeName for AppBarToggleButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarToggleButtonTemplateSettings";
}
impl ::core::convert::From<AppBarToggleButtonTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: AppBarToggleButtonTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBarToggleButtonTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &AppBarToggleButtonTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBarToggleButtonTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: AppBarToggleButtonTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBarToggleButtonTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &AppBarToggleButtonTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppBarToggleButtonTemplateSettings> for super::super::DependencyObject {
    fn from(value: AppBarToggleButtonTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&AppBarToggleButtonTemplateSettings> for super::super::DependencyObject {
    fn from(value: &AppBarToggleButtonTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for AppBarToggleButtonTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarToggleButtonTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ButtonBase(pub ::windows::core::IInspectable);
impl ButtonBase {
    pub fn ClickMode(&self) -> ::windows::core::Result<super::ClickMode> {
        let this = self;
        unsafe {
            let mut result__: super::ClickMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ClickMode>(result__)
        }
    }
    pub fn SetClickMode(&self, value: super::ClickMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsPointerOver(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Command(&self) -> ::windows::core::Result<super::super::Input::ICommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ICommand>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetCommand<'a, Param0: ::windows::core::IntoParam<'a, super::super::Input::ICommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetCommandParameter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Click<'a, Param0: ::windows::core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ClickModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsPointerOverProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsPressedProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CommandProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CommandParameterProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IButtonBaseStatics<R, F: FnOnce(&IButtonBaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ButtonBase, IButtonBaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ButtonBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ButtonBase;{fa002c1a-494e-46cf-91d4-e14a8d798674})");
}
unsafe impl ::windows::core::Interface for ButtonBase {
    type Vtable = IButtonBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa002c1a_494e_46cf_91d4_e14a8d798674);
}
impl ::windows::core::RuntimeName for ButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ButtonBase";
}
impl ::core::convert::From<ButtonBase> for ::windows::core::IUnknown {
    fn from(value: ButtonBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ButtonBase> for ::windows::core::IUnknown {
    fn from(value: &ButtonBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ButtonBase> for ::windows::core::IInspectable {
    fn from(value: ButtonBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ButtonBase> for ::windows::core::IInspectable {
    fn from(value: &ButtonBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ButtonBase> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: ButtonBase) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ButtonBase> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &ButtonBase) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ButtonBase> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: ButtonBase) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ButtonBase> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &ButtonBase) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ButtonBase> for super::ContentControl {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::Into::<super::ContentControl>::into(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::ContentControl {
    fn from(value: &ButtonBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for &ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ButtonBase> for super::Control {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::Control {
    fn from(value: &ButtonBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ButtonBase> for super::super::FrameworkElement {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::super::FrameworkElement {
    fn from(value: &ButtonBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ButtonBase> for super::super::UIElement {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::super::UIElement {
    fn from(value: &ButtonBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ButtonBase> for super::super::DependencyObject {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::super::DependencyObject {
    fn from(value: &ButtonBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ButtonBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ButtonBase {}
unsafe impl ::core::marker::Sync for ButtonBase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CalendarPanel(pub ::windows::core::IInspectable);
impl CalendarPanel {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CalendarPanel, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CalendarPanel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CalendarPanel;{fcd55a2d-02d3-4ee6-9a90-9df3ead00994})");
}
unsafe impl ::windows::core::Interface for CalendarPanel {
    type Vtable = ICalendarPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcd55a2d_02d3_4ee6_9a90_9df3ead00994);
}
impl ::windows::core::RuntimeName for CalendarPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CalendarPanel";
}
impl ::core::convert::From<CalendarPanel> for ::windows::core::IUnknown {
    fn from(value: CalendarPanel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CalendarPanel> for ::windows::core::IUnknown {
    fn from(value: &CalendarPanel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CalendarPanel> for ::windows::core::IInspectable {
    fn from(value: CalendarPanel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CalendarPanel> for ::windows::core::IInspectable {
    fn from(value: &CalendarPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CalendarPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CalendarPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CalendarPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CalendarPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CalendarPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: CalendarPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CalendarPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &CalendarPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<CalendarPanel> for super::Panel {
    fn from(value: CalendarPanel) -> Self {
        ::core::convert::Into::<super::Panel>::into(&value)
    }
}
impl ::core::convert::From<&CalendarPanel> for super::Panel {
    fn from(value: &CalendarPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for &CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CalendarPanel> for super::super::FrameworkElement {
    fn from(value: CalendarPanel) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&CalendarPanel> for super::super::FrameworkElement {
    fn from(value: &CalendarPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CalendarPanel> for super::super::UIElement {
    fn from(value: CalendarPanel) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&CalendarPanel> for super::super::UIElement {
    fn from(value: &CalendarPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CalendarPanel> for super::super::DependencyObject {
    fn from(value: CalendarPanel) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CalendarPanel> for super::super::DependencyObject {
    fn from(value: &CalendarPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CalendarPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CalendarPanel {}
unsafe impl ::core::marker::Sync for CalendarPanel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CalendarViewTemplateSettings(pub ::windows::core::IInspectable);
impl CalendarViewTemplateSettings {
    pub fn MinViewWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn HeaderText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn WeekDay1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn WeekDay2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn WeekDay3(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn WeekDay4(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn WeekDay5(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn WeekDay6(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn WeekDay7(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn HasMoreContentAfter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasMoreContentBefore(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasMoreViews(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CenterX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CenterY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CalendarViewTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CalendarViewTemplateSettings;{56c71483-64e1-477c-8a0b-cb2f3334b9b0})");
}
unsafe impl ::windows::core::Interface for CalendarViewTemplateSettings {
    type Vtable = ICalendarViewTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56c71483_64e1_477c_8a0b_cb2f3334b9b0);
}
impl ::windows::core::RuntimeName for CalendarViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CalendarViewTemplateSettings";
}
impl ::core::convert::From<CalendarViewTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: CalendarViewTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CalendarViewTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &CalendarViewTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CalendarViewTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: CalendarViewTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CalendarViewTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &CalendarViewTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CalendarViewTemplateSettings> for super::super::DependencyObject {
    fn from(value: CalendarViewTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CalendarViewTemplateSettings> for super::super::DependencyObject {
    fn from(value: &CalendarViewTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CalendarViewTemplateSettings {}
unsafe impl ::core::marker::Sync for CalendarViewTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CarouselPanel(pub ::windows::core::IInspectable);
impl CarouselPanel {
    pub fn CanVerticallyScroll(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanVerticallyScroll(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CanHorizontallyScroll(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanHorizontallyScroll(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ExtentWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExtentHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ViewportWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ViewportHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn HorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ScrollOwner(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetScrollOwner<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn LineUp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LineDown(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LineLeft(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LineRight(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PageUp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PageDown(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PageLeft(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PageRight(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MouseWheelUp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MouseWheelDown(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MouseWheelLeft(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MouseWheelRight(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetHorizontalOffset(&self, offset: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), offset).ok() }
    }
    pub fn SetVerticalOffset(&self, offset: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), offset).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MakeVisible<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>>(&self, visual: Param0, rectangle: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), visual.into_param().abi(), rectangle.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), orientation, alignment, &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), orientation, alignment, offset, &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn new() -> ::windows::core::Result<CarouselPanel> {
        Self::ICarouselPanelFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<CarouselPanel>(result__)
        })
    }
    pub fn ICarouselPanelFactory<R, F: FnOnce(&ICarouselPanelFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CarouselPanel, ICarouselPanelFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CarouselPanel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CarouselPanel;{deab78b2-373b-4151-8785-e544d0d9362b})");
}
unsafe impl ::windows::core::Interface for CarouselPanel {
    type Vtable = ICarouselPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdeab78b2_373b_4151_8785_e544d0d9362b);
}
impl ::windows::core::RuntimeName for CarouselPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CarouselPanel";
}
impl ::core::convert::From<CarouselPanel> for ::windows::core::IUnknown {
    fn from(value: CarouselPanel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CarouselPanel> for ::windows::core::IUnknown {
    fn from(value: &CarouselPanel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CarouselPanel> for ::windows::core::IInspectable {
    fn from(value: CarouselPanel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CarouselPanel> for ::windows::core::IInspectable {
    fn from(value: &CarouselPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CarouselPanel> for IScrollSnapPointsInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: CarouselPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CarouselPanel> for IScrollSnapPointsInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &CarouselPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IScrollSnapPointsInfo> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, IScrollSnapPointsInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IScrollSnapPointsInfo> for &CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, IScrollSnapPointsInfo> {
        ::core::convert::TryInto::<IScrollSnapPointsInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CarouselPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CarouselPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CarouselPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CarouselPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CarouselPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: CarouselPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CarouselPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &CarouselPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<CarouselPanel> for super::VirtualizingPanel {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::Into::<super::VirtualizingPanel>::into(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::VirtualizingPanel {
    fn from(value: &CarouselPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::VirtualizingPanel> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::VirtualizingPanel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::VirtualizingPanel>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::VirtualizingPanel> for &CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::VirtualizingPanel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::VirtualizingPanel>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CarouselPanel> for super::Panel {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::Into::<super::Panel>::into(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::Panel {
    fn from(value: &CarouselPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for &CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CarouselPanel> for super::super::FrameworkElement {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::super::FrameworkElement {
    fn from(value: &CarouselPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CarouselPanel> for super::super::UIElement {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::super::UIElement {
    fn from(value: &CarouselPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CarouselPanel> for super::super::DependencyObject {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::super::DependencyObject {
    fn from(value: &CarouselPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CarouselPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CarouselPanel {}
unsafe impl ::core::marker::Sync for CarouselPanel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ColorPickerSlider(pub ::windows::core::IInspectable);
impl ColorPickerSlider {
    pub fn ColorChannel(&self) -> ::windows::core::Result<super::ColorPickerHsvChannel> {
        let this = self;
        unsafe {
            let mut result__: super::ColorPickerHsvChannel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ColorPickerHsvChannel>(result__)
        }
    }
    pub fn SetColorChannel(&self, value: super::ColorPickerHsvChannel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ColorChannelProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorPickerSliderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<ColorPickerSlider> {
        Self::IColorPickerSliderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ColorPickerSlider>(result__)
        })
    }
    pub fn IColorPickerSliderStatics<R, F: FnOnce(&IColorPickerSliderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorPickerSlider, IColorPickerSliderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IColorPickerSliderFactory<R, F: FnOnce(&IColorPickerSliderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorPickerSlider, IColorPickerSliderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ColorPickerSlider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ColorPickerSlider;{94394d83-e0df-4c5f-bbcd-8155f4020440})");
}
unsafe impl ::windows::core::Interface for ColorPickerSlider {
    type Vtable = IColorPickerSlider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94394d83_e0df_4c5f_bbcd_8155f4020440);
}
impl ::windows::core::RuntimeName for ColorPickerSlider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ColorPickerSlider";
}
impl ::core::convert::From<ColorPickerSlider> for ::windows::core::IUnknown {
    fn from(value: ColorPickerSlider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ColorPickerSlider> for ::windows::core::IUnknown {
    fn from(value: &ColorPickerSlider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ColorPickerSlider> for ::windows::core::IInspectable {
    fn from(value: ColorPickerSlider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ColorPickerSlider> for ::windows::core::IInspectable {
    fn from(value: &ColorPickerSlider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ColorPickerSlider> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: ColorPickerSlider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ColorPickerSlider> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &ColorPickerSlider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ColorPickerSlider> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: ColorPickerSlider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ColorPickerSlider> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &ColorPickerSlider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::Slider {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::Into::<super::Slider>::into(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::Slider {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Slider> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Slider> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Slider>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Slider> for &ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Slider> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Slider>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorPickerSlider> for RangeBase {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::Into::<RangeBase>::into(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for RangeBase {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, RangeBase> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, RangeBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<RangeBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, RangeBase> for &ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, RangeBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<RangeBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::Control {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::Control {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::super::FrameworkElement {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::super::FrameworkElement {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::super::UIElement {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::super::UIElement {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::super::DependencyObject {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::super::DependencyObject {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ColorPickerSlider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ColorPickerSlider {}
unsafe impl ::core::marker::Sync for ColorPickerSlider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ColorSpectrum(pub ::windows::core::IInspectable);
impl ColorSpectrum {
    pub fn Color(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn HsvColor(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Numerics::Vector4 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Numerics::Vector4>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetHsvColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Numerics::Vector4>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn MinHue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetMinHue(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MaxHue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetMaxHue(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MinSaturation(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetMinSaturation(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MaxSaturation(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetMaxSaturation(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MinValue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetMinValue(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MaxValue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetMaxValue(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Shape(&self) -> ::windows::core::Result<super::ColorSpectrumShape> {
        let this = self;
        unsafe {
            let mut result__: super::ColorSpectrumShape = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ColorSpectrumShape>(result__)
        }
    }
    pub fn SetShape(&self, value: super::ColorSpectrumShape) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Components(&self) -> ::windows::core::Result<super::ColorSpectrumComponents> {
        let this = self;
        unsafe {
            let mut result__: super::ColorSpectrumComponents = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ColorSpectrumComponents>(result__)
        }
    }
    pub fn SetComponents(&self, value: super::ColorSpectrumComponents) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ColorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveColorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ColorProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn HsvColorProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MinHueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MaxHueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MinSaturationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MaxSaturationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MinValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MaxValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ShapeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ComponentsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<ColorSpectrum> {
        Self::IColorSpectrumFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ColorSpectrum>(result__)
        })
    }
    pub fn IColorSpectrumStatics<R, F: FnOnce(&IColorSpectrumStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorSpectrum, IColorSpectrumStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IColorSpectrumFactory<R, F: FnOnce(&IColorSpectrumFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorSpectrum, IColorSpectrumFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ColorSpectrum {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ColorSpectrum;{ce46f271-f509-4f98-8288-e4942fb385df})");
}
unsafe impl ::windows::core::Interface for ColorSpectrum {
    type Vtable = IColorSpectrum_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce46f271_f509_4f98_8288_e4942fb385df);
}
impl ::windows::core::RuntimeName for ColorSpectrum {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ColorSpectrum";
}
impl ::core::convert::From<ColorSpectrum> for ::windows::core::IUnknown {
    fn from(value: ColorSpectrum) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ColorSpectrum> for ::windows::core::IUnknown {
    fn from(value: &ColorSpectrum) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ColorSpectrum> for ::windows::core::IInspectable {
    fn from(value: ColorSpectrum) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ColorSpectrum> for ::windows::core::IInspectable {
    fn from(value: &ColorSpectrum) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ColorSpectrum> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: ColorSpectrum) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ColorSpectrum> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &ColorSpectrum) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ColorSpectrum> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: ColorSpectrum) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ColorSpectrum> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &ColorSpectrum) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ColorSpectrum> for super::Control {
    fn from(value: ColorSpectrum) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&ColorSpectrum> for super::Control {
    fn from(value: &ColorSpectrum) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorSpectrum> for super::super::FrameworkElement {
    fn from(value: ColorSpectrum) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&ColorSpectrum> for super::super::FrameworkElement {
    fn from(value: &ColorSpectrum) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorSpectrum> for super::super::UIElement {
    fn from(value: ColorSpectrum) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&ColorSpectrum> for super::super::UIElement {
    fn from(value: &ColorSpectrum) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorSpectrum> for super::super::DependencyObject {
    fn from(value: ColorSpectrum) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ColorSpectrum> for super::super::DependencyObject {
    fn from(value: &ColorSpectrum) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ColorSpectrum {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ColorSpectrum {}
unsafe impl ::core::marker::Sync for ColorSpectrum {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ComboBoxTemplateSettings(pub ::windows::core::IInspectable);
impl ComboBoxTemplateSettings {
    pub fn DropDownOpenedHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn DropDownClosedHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn DropDownOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SelectedItemDirection(&self) -> ::windows::core::Result<AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__: AnimationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AnimationDirection>(result__)
        }
    }
    pub fn DropDownContentMinWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IComboBoxTemplateSettings2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ComboBoxTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ComboBoxTemplateSettings;{83285c4e-17f6-4aa3-b61b-e87c718604ea})");
}
unsafe impl ::windows::core::Interface for ComboBoxTemplateSettings {
    type Vtable = IComboBoxTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83285c4e_17f6_4aa3_b61b_e87c718604ea);
}
impl ::windows::core::RuntimeName for ComboBoxTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ComboBoxTemplateSettings";
}
impl ::core::convert::From<ComboBoxTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: ComboBoxTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ComboBoxTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &ComboBoxTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ComboBoxTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: ComboBoxTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ComboBoxTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &ComboBoxTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ComboBoxTemplateSettings> for super::super::DependencyObject {
    fn from(value: ComboBoxTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ComboBoxTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ComboBoxTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ComboBoxTemplateSettings {}
unsafe impl ::core::marker::Sync for ComboBoxTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CommandBarFlyoutCommandBar(pub ::windows::core::IInspectable);
impl CommandBarFlyoutCommandBar {
    pub fn FlyoutTemplateSettings(&self) -> ::windows::core::Result<CommandBarFlyoutCommandBarTemplateSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CommandBarFlyoutCommandBarTemplateSettings>(result__)
        }
    }
    pub fn new() -> ::windows::core::Result<CommandBarFlyoutCommandBar> {
        Self::ICommandBarFlyoutCommandBarFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<CommandBarFlyoutCommandBar>(result__)
        })
    }
    pub fn ICommandBarFlyoutCommandBarFactory<R, F: FnOnce(&ICommandBarFlyoutCommandBarFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommandBarFlyoutCommandBar, ICommandBarFlyoutCommandBarFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CommandBarFlyoutCommandBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar;{14146e7c-38c4-55c4-b706-ce18f6061e7e})");
}
unsafe impl ::windows::core::Interface for CommandBarFlyoutCommandBar {
    type Vtable = ICommandBarFlyoutCommandBar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14146e7c_38c4_55c4_b706_ce18f6061e7e);
}
impl ::windows::core::RuntimeName for CommandBarFlyoutCommandBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar";
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for ::windows::core::IUnknown {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for ::windows::core::IUnknown {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for ::windows::core::IInspectable {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for ::windows::core::IInspectable {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CommandBarFlyoutCommandBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandBarFlyoutCommandBar) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CommandBarFlyoutCommandBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandBarFlyoutCommandBar) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CommandBarFlyoutCommandBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandBarFlyoutCommandBar) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CommandBarFlyoutCommandBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandBarFlyoutCommandBar) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::CommandBar {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::Into::<super::CommandBar>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::CommandBar {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CommandBar> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::CommandBar> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CommandBar>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CommandBar> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::CommandBar> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CommandBar>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::AppBar {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::Into::<super::AppBar>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::AppBar {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::AppBar> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::AppBar> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::AppBar>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::AppBar> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::AppBar> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::AppBar>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::ContentControl {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::Into::<super::ContentControl>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::ContentControl {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::Control {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::Control {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::super::FrameworkElement {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::super::FrameworkElement {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::super::UIElement {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::super::UIElement {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::super::DependencyObject {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::super::DependencyObject {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CommandBarFlyoutCommandBar {}
unsafe impl ::core::marker::Sync for CommandBarFlyoutCommandBar {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CommandBarFlyoutCommandBarTemplateSettings(pub ::windows::core::IInspectable);
impl CommandBarFlyoutCommandBarTemplateSettings {
    pub fn OpenAnimationStartPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OpenAnimationEndPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CloseAnimationEndPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CurrentWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandedWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionDelta(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionAnimationStartPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionAnimationEndPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionMoreButtonAnimationStartPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionMoreButtonAnimationEndPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandUpOverflowVerticalPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandDownOverflowVerticalPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandUpAnimationStartPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandUpAnimationEndPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandUpAnimationHoldPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandDownAnimationStartPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandDownAnimationEndPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandDownAnimationHoldPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OverflowContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CommandBarFlyoutCommandBarTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBarTemplateSettings;{47642c44-26ff-5d14-9cfc-77dc64f3a447})");
}
unsafe impl ::windows::core::Interface for CommandBarFlyoutCommandBarTemplateSettings {
    type Vtable = ICommandBarFlyoutCommandBarTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47642c44_26ff_5d14_9cfc_77dc64f3a447);
}
impl ::windows::core::RuntimeName for CommandBarFlyoutCommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBarTemplateSettings";
}
impl ::core::convert::From<CommandBarFlyoutCommandBarTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBarTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBarTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBarTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: &CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CommandBarFlyoutCommandBarTemplateSettings {}
unsafe impl ::core::marker::Sync for CommandBarFlyoutCommandBarTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CommandBarTemplateSettings(pub ::windows::core::IInspectable);
impl CommandBarTemplateSettings {
    pub fn ContentHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OverflowContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn OverflowContentMinWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentMaxHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeOverflowContentHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentMaxWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICommandBarTemplateSettings2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn EffectiveOverflowButtonVisibility(&self) -> ::windows::core::Result<super::super::Visibility> {
        let this = &::windows::core::Interface::cast::<ICommandBarTemplateSettings3>(self)?;
        unsafe {
            let mut result__: super::super::Visibility = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Visibility>(result__)
        }
    }
    pub fn OverflowContentCompactYTranslation(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICommandBarTemplateSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentMinimalYTranslation(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICommandBarTemplateSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentHiddenYTranslation(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ICommandBarTemplateSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CommandBarTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarTemplateSettings;{61c8f92c-05aa-414a-a2ae-482c5a46c08e})");
}
unsafe impl ::windows::core::Interface for CommandBarTemplateSettings {
    type Vtable = ICommandBarTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61c8f92c_05aa_414a_a2ae_482c5a46c08e);
}
impl ::windows::core::RuntimeName for CommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarTemplateSettings";
}
impl ::core::convert::From<CommandBarTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: CommandBarTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CommandBarTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &CommandBarTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CommandBarTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: CommandBarTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CommandBarTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &CommandBarTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CommandBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: CommandBarTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CommandBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: &CommandBarTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CommandBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CommandBarTemplateSettings {}
unsafe impl ::core::marker::Sync for CommandBarTemplateSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ComponentResourceLocation(pub i32);
impl ComponentResourceLocation {
    pub const Application: ComponentResourceLocation = ComponentResourceLocation(0i32);
    pub const Nested: ComponentResourceLocation = ComponentResourceLocation(1i32);
}
impl ::core::convert::From<i32> for ComponentResourceLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ComponentResourceLocation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ComponentResourceLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ComponentResourceLocation;i4)");
}
impl ::windows::core::DefaultType for ComponentResourceLocation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragCompletedEventArgs(pub ::windows::core::IInspectable);
impl DragCompletedEventArgs {
    pub fn HorizontalChange(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalChange(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn Canceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled(horizontalchange: f64, verticalchange: f64, canceled: bool) -> ::windows::core::Result<DragCompletedEventArgs> {
        Self::IDragCompletedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), horizontalchange, verticalchange, canceled, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<DragCompletedEventArgs>(result__)
        })
    }
    pub fn IDragCompletedEventArgsFactory<R, F: FnOnce(&IDragCompletedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragCompletedEventArgs, IDragCompletedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DragCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.DragCompletedEventArgs;{b04f29a1-bd16-48f6-a511-9c2763641331})");
}
unsafe impl ::windows::core::Interface for DragCompletedEventArgs {
    type Vtable = IDragCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb04f29a1_bd16_48f6_a511_9c2763641331);
}
impl ::windows::core::RuntimeName for DragCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragCompletedEventArgs";
}
impl ::core::convert::From<DragCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DragCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DragCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DragCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DragCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DragCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DragCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DragCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DragCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DragCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DragCompletedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: DragCompletedEventArgs) -> Self {
        ::core::convert::Into::<super::super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&DragCompletedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &DragCompletedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for DragCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for &DragCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DragCompletedEventArgs {}
unsafe impl ::core::marker::Sync for DragCompletedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragCompletedEventHandler(::windows::core::IUnknown);
impl DragCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragCompletedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DragCompletedEventHandler_box::<F> {
            vtable: &DragCompletedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, DragCompletedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DragCompletedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({36b28888-19ac-4b4e-9137-a6cf2b023883})");
}
unsafe impl ::windows::core::Interface for DragCompletedEventHandler {
    type Vtable = DragCompletedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36b28888_19ac_4b4e_9137_a6cf2b023883);
}
#[repr(C)]
#[doc(hidden)]
pub struct DragCompletedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct DragCompletedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragCompletedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const DragCompletedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragCompletedEventArgs>) -> ::windows::core::Result<()> + 'static> DragCompletedEventHandler_box<F> {
    const VTABLE: DragCompletedEventHandler_abi = DragCompletedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DragCompletedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <DragCompletedEventArgs as ::windows::core::Abi>::Abi as *const <DragCompletedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragDeltaEventArgs(pub ::windows::core::IInspectable);
impl DragDeltaEventArgs {
    pub fn HorizontalChange(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalChange(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CreateInstanceWithHorizontalChangeAndVerticalChange(horizontalchange: f64, verticalchange: f64) -> ::windows::core::Result<DragDeltaEventArgs> {
        Self::IDragDeltaEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), horizontalchange, verticalchange, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<DragDeltaEventArgs>(result__)
        })
    }
    pub fn IDragDeltaEventArgsFactory<R, F: FnOnce(&IDragDeltaEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragDeltaEventArgs, IDragDeltaEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DragDeltaEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.DragDeltaEventArgs;{2c2dd73c-2806-49fc-aae9-6d792b572b6a})");
}
unsafe impl ::windows::core::Interface for DragDeltaEventArgs {
    type Vtable = IDragDeltaEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c2dd73c_2806_49fc_aae9_6d792b572b6a);
}
impl ::windows::core::RuntimeName for DragDeltaEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragDeltaEventArgs";
}
impl ::core::convert::From<DragDeltaEventArgs> for ::windows::core::IUnknown {
    fn from(value: DragDeltaEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DragDeltaEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DragDeltaEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragDeltaEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DragDeltaEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DragDeltaEventArgs> for ::windows::core::IInspectable {
    fn from(value: DragDeltaEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DragDeltaEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DragDeltaEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragDeltaEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DragDeltaEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DragDeltaEventArgs> for super::super::RoutedEventArgs {
    fn from(value: DragDeltaEventArgs) -> Self {
        ::core::convert::Into::<super::super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&DragDeltaEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &DragDeltaEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for DragDeltaEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for &DragDeltaEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DragDeltaEventArgs {}
unsafe impl ::core::marker::Sync for DragDeltaEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragDeltaEventHandler(::windows::core::IUnknown);
impl DragDeltaEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragDeltaEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DragDeltaEventHandler_box::<F> {
            vtable: &DragDeltaEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, DragDeltaEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DragDeltaEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({4ac24f9f-ac28-49e9-9189-dccffeb66472})");
}
unsafe impl ::windows::core::Interface for DragDeltaEventHandler {
    type Vtable = DragDeltaEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ac24f9f_ac28_49e9_9189_dccffeb66472);
}
#[repr(C)]
#[doc(hidden)]
pub struct DragDeltaEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct DragDeltaEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragDeltaEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const DragDeltaEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragDeltaEventArgs>) -> ::windows::core::Result<()> + 'static> DragDeltaEventHandler_box<F> {
    const VTABLE: DragDeltaEventHandler_abi = DragDeltaEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DragDeltaEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <DragDeltaEventArgs as ::windows::core::Abi>::Abi as *const <DragDeltaEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragStartedEventArgs(pub ::windows::core::IInspectable);
impl DragStartedEventArgs {
    pub fn HorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CreateInstanceWithHorizontalOffsetAndVerticalOffset(horizontaloffset: f64, verticaloffset: f64) -> ::windows::core::Result<DragStartedEventArgs> {
        Self::IDragStartedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), horizontaloffset, verticaloffset, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<DragStartedEventArgs>(result__)
        })
    }
    pub fn IDragStartedEventArgsFactory<R, F: FnOnce(&IDragStartedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragStartedEventArgs, IDragStartedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DragStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.DragStartedEventArgs;{9f915dd0-a124-4366-bd85-2408214aeed4})");
}
unsafe impl ::windows::core::Interface for DragStartedEventArgs {
    type Vtable = IDragStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f915dd0_a124_4366_bd85_2408214aeed4);
}
impl ::windows::core::RuntimeName for DragStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragStartedEventArgs";
}
impl ::core::convert::From<DragStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DragStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DragStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DragStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DragStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DragStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DragStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DragStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DragStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DragStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DragStartedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: DragStartedEventArgs) -> Self {
        ::core::convert::Into::<super::super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&DragStartedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &DragStartedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for DragStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for &DragStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DragStartedEventArgs {}
unsafe impl ::core::marker::Sync for DragStartedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragStartedEventHandler(::windows::core::IUnknown);
impl DragStartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragStartedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DragStartedEventHandler_box::<F> {
            vtable: &DragStartedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, DragStartedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DragStartedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({d2eea48a-c65a-495d-a2f1-72c66989142d})");
}
unsafe impl ::windows::core::Interface for DragStartedEventHandler {
    type Vtable = DragStartedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2eea48a_c65a_495d_a2f1_72c66989142d);
}
#[repr(C)]
#[doc(hidden)]
pub struct DragStartedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct DragStartedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragStartedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const DragStartedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DragStartedEventArgs>) -> ::windows::core::Result<()> + 'static> DragStartedEventHandler_box<F> {
    const VTABLE: DragStartedEventHandler_abi = DragStartedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DragStartedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <DragStartedEventArgs as ::windows::core::Abi>::Abi as *const <DragStartedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EdgeTransitionLocation(pub i32);
impl EdgeTransitionLocation {
    pub const Left: EdgeTransitionLocation = EdgeTransitionLocation(0i32);
    pub const Top: EdgeTransitionLocation = EdgeTransitionLocation(1i32);
    pub const Right: EdgeTransitionLocation = EdgeTransitionLocation(2i32);
    pub const Bottom: EdgeTransitionLocation = EdgeTransitionLocation(3i32);
}
impl ::core::convert::From<i32> for EdgeTransitionLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EdgeTransitionLocation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EdgeTransitionLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.EdgeTransitionLocation;i4)");
}
impl ::windows::core::DefaultType for EdgeTransitionLocation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FlyoutBase(pub ::windows::core::IInspectable);
impl FlyoutBase {
    pub fn Placement(&self) -> ::windows::core::Result<FlyoutPlacementMode> {
        let this = self;
        unsafe {
            let mut result__: FlyoutPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FlyoutPlacementMode>(result__)
        }
    }
    pub fn SetPlacement(&self, value: FlyoutPlacementMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Opened<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Opening<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpening<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ShowAt<'a, Param0: ::windows::core::IntoParam<'a, super::super::FrameworkElement>>(&self, placementtarget: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), placementtarget.into_param().abi()).ok() }
    }
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PlacementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn AttachedFlyoutProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetAttachedFlyout<'a, Param0: ::windows::core::IntoParam<'a, super::super::FrameworkElement>>(element: Param0) -> ::windows::core::Result<FlyoutBase> {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<FlyoutBase>(result__)
        })
    }
    pub fn SetAttachedFlyout<'a, Param0: ::windows::core::IntoParam<'a, super::super::FrameworkElement>, Param1: ::windows::core::IntoParam<'a, FlyoutBase>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IFlyoutBaseStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn ShowAttachedFlyout<'a, Param0: ::windows::core::IntoParam<'a, super::super::FrameworkElement>>(flyoutowner: Param0) -> ::windows::core::Result<()> {
        Self::IFlyoutBaseStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), flyoutowner.into_param().abi()).ok() })
    }
    pub fn Target(&self) -> ::windows::core::Result<super::super::FrameworkElement> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::FrameworkElement>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LightDismissOverlayMode(&self) -> ::windows::core::Result<super::LightDismissOverlayMode> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__: super::LightDismissOverlayMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::LightDismissOverlayMode>(result__)
        }
    }
    pub fn SetLightDismissOverlayMode(&self, value: super::LightDismissOverlayMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ElementSoundMode(&self) -> ::windows::core::Result<super::super::ElementSoundMode> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__: super::super::ElementSoundMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ElementSoundMode>(result__)
        }
    }
    pub fn SetElementSoundMode(&self, value: super::super::ElementSoundMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Closing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn AllowFocusOnInteractionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn LightDismissOverlayModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn AllowFocusWhenDisabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ElementSoundModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OverlayInputPassThroughElement(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetOverlayInputPassThroughElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn OverlayInputPassThroughElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<'a, Param0: ::windows::core::IntoParam<'a, super::super::Input::ProcessKeyboardAcceleratorEventArgs>>(&self, args: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), args.into_param().abi()).ok() }
    }
    pub fn ShowMode(&self) -> ::windows::core::Result<FlyoutShowMode> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__: FlyoutShowMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FlyoutShowMode>(result__)
        }
    }
    pub fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InputDevicePrefersPrimaryCommands(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AreOpenCloseAnimationsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAreOpenCloseAnimationsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ShowAt2<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, FlyoutShowOptions>>(&self, placementtarget: Param0, showoptions: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), placementtarget.into_param().abi(), showoptions.into_param().abi()).ok() }
    }
    pub fn TargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ShowModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn InputDevicePrefersPrimaryCommandsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn AreOpenCloseAnimationsEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsOpenProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ShouldConstrainToRootBounds(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldConstrainToRootBounds(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsConstrainedToRootBounds(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::super::XamlRoot>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ShouldConstrainToRootBoundsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics6(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IFlyoutBaseStatics<R, F: FnOnce(&IFlyoutBaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FlyoutBase, IFlyoutBaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFlyoutBaseStatics2<R, F: FnOnce(&IFlyoutBaseStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FlyoutBase, IFlyoutBaseStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFlyoutBaseStatics3<R, F: FnOnce(&IFlyoutBaseStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FlyoutBase, IFlyoutBaseStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFlyoutBaseStatics5<R, F: FnOnce(&IFlyoutBaseStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FlyoutBase, IFlyoutBaseStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFlyoutBaseStatics6<R, F: FnOnce(&IFlyoutBaseStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FlyoutBase, IFlyoutBaseStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FlyoutBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutBase;{723eea0b-d12e-430d-a9f0-9bb32bbf9913})");
}
unsafe impl ::windows::core::Interface for FlyoutBase {
    type Vtable = IFlyoutBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x723eea0b_d12e_430d_a9f0_9bb32bbf9913);
}
impl ::windows::core::RuntimeName for FlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutBase";
}
impl ::core::convert::From<FlyoutBase> for ::windows::core::IUnknown {
    fn from(value: FlyoutBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FlyoutBase> for ::windows::core::IUnknown {
    fn from(value: &FlyoutBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FlyoutBase> for ::windows::core::IInspectable {
    fn from(value: FlyoutBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FlyoutBase> for ::windows::core::IInspectable {
    fn from(value: &FlyoutBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FlyoutBase> for super::super::DependencyObject {
    fn from(value: FlyoutBase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&FlyoutBase> for super::super::DependencyObject {
    fn from(value: &FlyoutBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for FlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &FlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for FlyoutBase {}
unsafe impl ::core::marker::Sync for FlyoutBase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FlyoutBaseClosingEventArgs(pub ::windows::core::IInspectable);
impl FlyoutBaseClosingEventArgs {
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FlyoutBaseClosingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutBaseClosingEventArgs;{d075852d-b09a-4fd1-b005-db2ba01206fb})");
}
unsafe impl ::windows::core::Interface for FlyoutBaseClosingEventArgs {
    type Vtable = IFlyoutBaseClosingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd075852d_b09a_4fd1_b005_db2ba01206fb);
}
impl ::windows::core::RuntimeName for FlyoutBaseClosingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutBaseClosingEventArgs";
}
impl ::core::convert::From<FlyoutBaseClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: FlyoutBaseClosingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FlyoutBaseClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FlyoutBaseClosingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FlyoutBaseClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FlyoutBaseClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FlyoutBaseClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: FlyoutBaseClosingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FlyoutBaseClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FlyoutBaseClosingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FlyoutBaseClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FlyoutBaseClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FlyoutBaseClosingEventArgs {}
unsafe impl ::core::marker::Sync for FlyoutBaseClosingEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FlyoutPlacementMode(pub i32);
impl FlyoutPlacementMode {
    pub const Top: FlyoutPlacementMode = FlyoutPlacementMode(0i32);
    pub const Bottom: FlyoutPlacementMode = FlyoutPlacementMode(1i32);
    pub const Left: FlyoutPlacementMode = FlyoutPlacementMode(2i32);
    pub const Right: FlyoutPlacementMode = FlyoutPlacementMode(3i32);
    pub const Full: FlyoutPlacementMode = FlyoutPlacementMode(4i32);
    pub const TopEdgeAlignedLeft: FlyoutPlacementMode = FlyoutPlacementMode(5i32);
    pub const TopEdgeAlignedRight: FlyoutPlacementMode = FlyoutPlacementMode(6i32);
    pub const BottomEdgeAlignedLeft: FlyoutPlacementMode = FlyoutPlacementMode(7i32);
    pub const BottomEdgeAlignedRight: FlyoutPlacementMode = FlyoutPlacementMode(8i32);
    pub const LeftEdgeAlignedTop: FlyoutPlacementMode = FlyoutPlacementMode(9i32);
    pub const LeftEdgeAlignedBottom: FlyoutPlacementMode = FlyoutPlacementMode(10i32);
    pub const RightEdgeAlignedTop: FlyoutPlacementMode = FlyoutPlacementMode(11i32);
    pub const RightEdgeAlignedBottom: FlyoutPlacementMode = FlyoutPlacementMode(12i32);
    pub const Auto: FlyoutPlacementMode = FlyoutPlacementMode(13i32);
}
impl ::core::convert::From<i32> for FlyoutPlacementMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FlyoutPlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FlyoutPlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.FlyoutPlacementMode;i4)");
}
impl ::windows::core::DefaultType for FlyoutPlacementMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FlyoutShowMode(pub i32);
impl FlyoutShowMode {
    pub const Auto: FlyoutShowMode = FlyoutShowMode(0i32);
    pub const Standard: FlyoutShowMode = FlyoutShowMode(1i32);
    pub const Transient: FlyoutShowMode = FlyoutShowMode(2i32);
    pub const TransientWithDismissOnPointerMoveAway: FlyoutShowMode = FlyoutShowMode(3i32);
}
impl ::core::convert::From<i32> for FlyoutShowMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FlyoutShowMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FlyoutShowMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.FlyoutShowMode;i4)");
}
impl ::windows::core::DefaultType for FlyoutShowMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FlyoutShowOptions(pub ::windows::core::IInspectable);
impl FlyoutShowOptions {
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExclusionRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExclusionRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ShowMode(&self) -> ::windows::core::Result<FlyoutShowMode> {
        let this = self;
        unsafe {
            let mut result__: FlyoutShowMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FlyoutShowMode>(result__)
        }
    }
    pub fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Placement(&self) -> ::windows::core::Result<FlyoutPlacementMode> {
        let this = self;
        unsafe {
            let mut result__: FlyoutPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FlyoutPlacementMode>(result__)
        }
    }
    pub fn SetPlacement(&self, value: FlyoutPlacementMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn new() -> ::windows::core::Result<FlyoutShowOptions> {
        Self::IFlyoutShowOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<FlyoutShowOptions>(result__)
        })
    }
    pub fn IFlyoutShowOptionsFactory<R, F: FnOnce(&IFlyoutShowOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FlyoutShowOptions, IFlyoutShowOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FlyoutShowOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutShowOptions;{57d693ad-0c74-54dd-b110-1ee43fabadd9})");
}
unsafe impl ::windows::core::Interface for FlyoutShowOptions {
    type Vtable = IFlyoutShowOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57d693ad_0c74_54dd_b110_1ee43fabadd9);
}
impl ::windows::core::RuntimeName for FlyoutShowOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutShowOptions";
}
impl ::core::convert::From<FlyoutShowOptions> for ::windows::core::IUnknown {
    fn from(value: FlyoutShowOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FlyoutShowOptions> for ::windows::core::IUnknown {
    fn from(value: &FlyoutShowOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FlyoutShowOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FlyoutShowOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FlyoutShowOptions> for ::windows::core::IInspectable {
    fn from(value: FlyoutShowOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FlyoutShowOptions> for ::windows::core::IInspectable {
    fn from(value: &FlyoutShowOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FlyoutShowOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FlyoutShowOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FlyoutShowOptions {}
unsafe impl ::core::marker::Sync for FlyoutShowOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeneratorDirection(pub i32);
impl GeneratorDirection {
    pub const Forward: GeneratorDirection = GeneratorDirection(0i32);
    pub const Backward: GeneratorDirection = GeneratorDirection(1i32);
}
impl ::core::convert::From<i32> for GeneratorDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GeneratorDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GeneratorDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.GeneratorDirection;i4)");
}
impl ::windows::core::DefaultType for GeneratorDirection {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GeneratorPosition {
    pub Index: i32,
    pub Offset: i32,
}
impl GeneratorPosition {}
impl ::core::default::Default for GeneratorPosition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GeneratorPosition {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GeneratorPosition").field("Index", &self.Index).field("Offset", &self.Offset).finish()
    }
}
impl ::core::cmp::PartialEq for GeneratorPosition {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for GeneratorPosition {}
unsafe impl ::windows::core::Abi for GeneratorPosition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GeneratorPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Controls.Primitives.GeneratorPosition;i4;i4)");
}
impl ::windows::core::DefaultType for GeneratorPosition {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeneratorPositionHelper(pub ::windows::core::IInspectable);
impl GeneratorPositionHelper {
    pub fn FromIndexAndOffset(index: i32, offset: i32) -> ::windows::core::Result<GeneratorPosition> {
        Self::IGeneratorPositionHelperStatics(|this| unsafe {
            let mut result__: GeneratorPosition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, offset, &mut result__).from_abi::<GeneratorPosition>(result__)
        })
    }
    pub fn IGeneratorPositionHelperStatics<R, F: FnOnce(&IGeneratorPositionHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeneratorPositionHelper, IGeneratorPositionHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GeneratorPositionHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.GeneratorPositionHelper;{cd40318d-7745-40d9-ab9d-abbda4a7ffea})");
}
unsafe impl ::windows::core::Interface for GeneratorPositionHelper {
    type Vtable = IGeneratorPositionHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd40318d_7745_40d9_ab9d_abbda4a7ffea);
}
impl ::windows::core::RuntimeName for GeneratorPositionHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GeneratorPositionHelper";
}
impl ::core::convert::From<GeneratorPositionHelper> for ::windows::core::IUnknown {
    fn from(value: GeneratorPositionHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeneratorPositionHelper> for ::windows::core::IUnknown {
    fn from(value: &GeneratorPositionHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeneratorPositionHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeneratorPositionHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeneratorPositionHelper> for ::windows::core::IInspectable {
    fn from(value: GeneratorPositionHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeneratorPositionHelper> for ::windows::core::IInspectable {
    fn from(value: &GeneratorPositionHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeneratorPositionHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeneratorPositionHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GeneratorPositionHelper {}
unsafe impl ::core::marker::Sync for GeneratorPositionHelper {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GridViewItemPresenter(pub ::windows::core::IInspectable);
impl GridViewItemPresenter {
    pub fn SelectionCheckMarkVisualEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelectionCheckMarkVisualEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckHintBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckHintBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckSelectingBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckSelectingBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn DragBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDragBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn DragForeground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDragForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PlaceholderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPlaceholderBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPointerOverBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedForeground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPointerOverBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPointerOverBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetSelectedBorderThickness<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisabledOpacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDisabledOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DragOpacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDragOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReorderHintOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetReorderHintOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterHorizontalContentAlignment(&self) -> ::windows::core::Result<super::super::HorizontalAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::super::HorizontalAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::HorizontalAlignment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGridViewItemPresenterHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterVerticalContentAlignment(&self) -> ::windows::core::Result<super::super::VerticalAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::super::VerticalAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::VerticalAlignment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGridViewItemPresenterVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterPadding(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGridViewItemPresenterPadding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PointerOverBackgroundMargin(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetPointerOverBackgroundMargin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContentMargin(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetContentMargin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectionCheckMarkVisualEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckHintBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckSelectingBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragForegroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FocusBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PlaceholderBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedForegroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPointerOverBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPointerOverBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBorderThicknessProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DisabledOpacityProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragOpacityProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ReorderHintOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterHorizontalContentAlignmentProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterVerticalContentAlignmentProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterPaddingProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBackgroundMarginProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentMarginProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<GridViewItemPresenter> {
        Self::IGridViewItemPresenterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<GridViewItemPresenter>(result__)
        })
    }
    pub fn IGridViewItemPresenterStatics<R, F: FnOnce(&IGridViewItemPresenterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GridViewItemPresenter, IGridViewItemPresenterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGridViewItemPresenterFactory<R, F: FnOnce(&IGridViewItemPresenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GridViewItemPresenter, IGridViewItemPresenterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GridViewItemPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.GridViewItemPresenter;{214f9010-56e2-4821-8a1c-2305709af94b})");
}
unsafe impl ::windows::core::Interface for GridViewItemPresenter {
    type Vtable = IGridViewItemPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x214f9010_56e2_4821_8a1c_2305709af94b);
}
impl ::windows::core::RuntimeName for GridViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GridViewItemPresenter";
}
impl ::core::convert::From<GridViewItemPresenter> for ::windows::core::IUnknown {
    fn from(value: GridViewItemPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for ::windows::core::IUnknown {
    fn from(value: &GridViewItemPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GridViewItemPresenter> for ::windows::core::IInspectable {
    fn from(value: GridViewItemPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for ::windows::core::IInspectable {
    fn from(value: &GridViewItemPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<GridViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: GridViewItemPresenter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&GridViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &GridViewItemPresenter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<GridViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: GridViewItemPresenter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&GridViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &GridViewItemPresenter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<GridViewItemPresenter> for super::ContentPresenter {
    fn from(value: GridViewItemPresenter) -> Self {
        ::core::convert::Into::<super::ContentPresenter>::into(&value)
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for super::ContentPresenter {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentPresenter> for GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentPresenter>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentPresenter> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentPresenter>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<GridViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: GridViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<GridViewItemPresenter> for super::super::UIElement {
    fn from(value: GridViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for super::super::UIElement {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<GridViewItemPresenter> for super::super::DependencyObject {
    fn from(value: GridViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for super::super::DependencyObject {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GridViewItemPresenter {}
unsafe impl ::core::marker::Sync for GridViewItemPresenter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GridViewItemTemplateSettings(pub ::windows::core::IInspectable);
impl GridViewItemTemplateSettings {
    pub fn DragItemsCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GridViewItemTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.GridViewItemTemplateSettings;{9e30baaf-165d-4267-a45e-1a43a75706ac})");
}
unsafe impl ::windows::core::Interface for GridViewItemTemplateSettings {
    type Vtable = IGridViewItemTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e30baaf_165d_4267_a45e_1a43a75706ac);
}
impl ::windows::core::RuntimeName for GridViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GridViewItemTemplateSettings";
}
impl ::core::convert::From<GridViewItemTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: GridViewItemTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GridViewItemTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &GridViewItemTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GridViewItemTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: GridViewItemTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GridViewItemTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &GridViewItemTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<GridViewItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: GridViewItemTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&GridViewItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: &GridViewItemTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GridViewItemTemplateSettings {}
unsafe impl ::core::marker::Sync for GridViewItemTemplateSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GroupHeaderPlacement(pub i32);
impl GroupHeaderPlacement {
    pub const Top: GroupHeaderPlacement = GroupHeaderPlacement(0i32);
    pub const Left: GroupHeaderPlacement = GroupHeaderPlacement(1i32);
}
impl ::core::convert::From<i32> for GroupHeaderPlacement {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GroupHeaderPlacement {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GroupHeaderPlacement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.GroupHeaderPlacement;i4)");
}
impl ::windows::core::DefaultType for GroupHeaderPlacement {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBarButtonTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBarButtonTemplateSettings {
    type Vtable = IAppBarButtonTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbc9b39d_0c95_4951_bff2_13963691c366);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarButtonTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBarTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBarTemplateSettings {
    type Vtable = IAppBarTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcc2a863_eb35_423c_8389_d7827be3bf67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBarTemplateSettings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBarTemplateSettings2 {
    type Vtable = IAppBarTemplateSettings2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbe66259_0399_5bcc_b925_4d5f5c9a4568);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarTemplateSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBarToggleButtonTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBarToggleButtonTemplateSettings {
    type Vtable = IAppBarToggleButtonTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaf99c48_d8f4_40d9_9fa3_3a64f0fec5d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarToggleButtonTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IButtonBase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IButtonBase {
    type Vtable = IButtonBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa002c1a_494e_46cf_91d4_e14a8d798674);
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::ClickMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::ClickMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IButtonBaseFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IButtonBaseFactory {
    type Vtable = IButtonBaseFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x389b7c71_5220_42b2_9992_2690c1a6702f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IButtonBaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IButtonBaseStatics {
    type Vtable = IButtonBaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67ef17e1_fe37_474f_9e97_3b5e0b30f2df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICalendarPanel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICalendarPanel {
    type Vtable = ICalendarPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcd55a2d_02d3_4ee6_9a90_9df3ead00994);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICalendarViewTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICalendarViewTemplateSettings {
    type Vtable = ICalendarViewTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56c71483_64e1_477c_8a0b_cb2f3334b9b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarViewTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICarouselPanel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICarouselPanel {
    type Vtable = ICarouselPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdeab78b2_373b_4151_8785_e544d0d9362b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICarouselPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICarouselPanelFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICarouselPanelFactory {
    type Vtable = ICarouselPanelFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1109404_9ae1_440e_a0dd_bbb6e2293cbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICarouselPanelFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorPickerSlider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorPickerSlider {
    type Vtable = IColorPickerSlider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94394d83_e0df_4c5f_bbcd_8155f4020440);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSlider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::ColorPickerHsvChannel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::ColorPickerHsvChannel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorPickerSliderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorPickerSliderFactory {
    type Vtable = IColorPickerSliderFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06d879a2_8c07_4b1e_a940_9fbce8f49639);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSliderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorPickerSliderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorPickerSliderStatics {
    type Vtable = IColorPickerSliderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22eafc6a_9fe3_4eee_8734_a1398ec4413a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSliderStatics_abi(
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
pub struct IColorSpectrum(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorSpectrum {
    type Vtable = IColorSpectrum_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce46f271_f509_4f98_8288_e4942fb385df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrum_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::ColorSpectrumShape) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::ColorSpectrumShape) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::ColorSpectrumComponents) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::ColorSpectrumComponents) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorSpectrumFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorSpectrumFactory {
    type Vtable = IColorSpectrumFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90c7e61e_904d_42ab_b44f_e68dbf0cdee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrumFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorSpectrumStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorSpectrumStatics {
    type Vtable = IColorSpectrumStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x906bee7c_2cee_4e90_968b_f0a5bd691b4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrumStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IComboBoxTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IComboBoxTemplateSettings {
    type Vtable = IComboBoxTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83285c4e_17f6_4aa3_b61b_e87c718604ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComboBoxTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AnimationDirection) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IComboBoxTemplateSettings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IComboBoxTemplateSettings2 {
    type Vtable = IComboBoxTemplateSettings2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00e90cd7_68be_449d_b5a7_76e26f703e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComboBoxTemplateSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBar(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandBarFlyoutCommandBar {
    type Vtable = ICommandBarFlyoutCommandBar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14146e7c_38c4_55c4_b706_ce18f6061e7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBar_abi(
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
pub struct ICommandBarFlyoutCommandBarFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandBarFlyoutCommandBarFactory {
    type Vtable = ICommandBarFlyoutCommandBarFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8236f9f_5559_5697_8e6f_20d70ca17dd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBarFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandBarFlyoutCommandBarTemplateSettings {
    type Vtable = ICommandBarFlyoutCommandBarTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47642c44_26ff_5d14_9cfc_77dc64f3a447);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandBarTemplateSettings {
    type Vtable = ICommandBarTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61c8f92c_05aa_414a_a2ae_482c5a46c08e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandBarTemplateSettings2 {
    type Vtable = ICommandBarTemplateSettings2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbb24f93_c2e2_4177_a2b6_3cd705073cf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandBarTemplateSettings3 {
    type Vtable = ICommandBarTemplateSettings3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bd71eba_3403_4bfe_842d_2ce8c511d245);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Visibility) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandBarTemplateSettings4 {
    type Vtable = ICommandBarTemplateSettings4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2562dd3_aa58_59c5_853b_828a19d1214e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragCompletedEventArgs {
    type Vtable = IDragCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb04f29a1_bd16_48f6_a511_9c2763641331);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragCompletedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragCompletedEventArgsFactory {
    type Vtable = IDragCompletedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36a7d99d_148c_495f_a0fc_afc871d62f33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragCompletedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragDeltaEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragDeltaEventArgs {
    type Vtable = IDragDeltaEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c2dd73c_2806_49fc_aae9_6d792b572b6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDeltaEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragDeltaEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragDeltaEventArgsFactory {
    type Vtable = IDragDeltaEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46e7a1ef_ae15_44a6_8a04_95b0bf9ab876);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDeltaEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, horizontalchange: f64, verticalchange: f64, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragStartedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragStartedEventArgs {
    type Vtable = IDragStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f915dd0_a124_4366_bd85_2408214aeed4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragStartedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragStartedEventArgsFactory {
    type Vtable = IDragStartedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5eefe579_c706_4781_a308_c9e7f4c6a1d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, horizontaloffset: f64, verticaloffset: f64, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBase {
    type Vtable = IFlyoutBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x723eea0b_d12e_430d_a9f0_9bb32bbf9913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FlyoutPlacementMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FlyoutPlacementMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, placementtarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBase2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBase2 {
    type Vtable = IFlyoutBase2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf82b435e_65b3_41c6_a9e2_77b67bc4c00c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::LightDismissOverlayMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::LightDismissOverlayMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::ElementSoundMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::ElementSoundMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBase3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBase3 {
    type Vtable = IFlyoutBase3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa89c9712_48e0_4240_95b9_0dfd0826a8d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBase4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBase4 {
    type Vtable = IFlyoutBase4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3897d69_a37f_4828_9b70_0ef67c03b5f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBase5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBase5 {
    type Vtable = IFlyoutBase5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad3ec0c7_12bb_5a73_b78e_105192ca73d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FlyoutShowMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FlyoutShowMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, placementtarget: ::windows::core::RawPtr, showoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBase6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBase6 {
    type Vtable = IFlyoutBase6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5399de8c_06cc_5b52_b65a_ff9322d1c940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBaseClosingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseClosingEventArgs {
    type Vtable = IFlyoutBaseClosingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd075852d_b09a_4fd1_b005_db2ba01206fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseClosingEventArgs_abi(
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
pub struct IFlyoutBaseFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseFactory {
    type Vtable = IFlyoutBaseFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c3363d7_fca7_407e_920e_70e15e9f0bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBaseOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseOverrides {
    type Vtable = IFlyoutBaseOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x101dec86_6f4d_45a4_9d0e_3ece6f16977e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseOverrides_abi(
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
pub struct IFlyoutBaseOverrides4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseOverrides4 {
    type Vtable = IFlyoutBaseOverrides4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6bfd04d_5ff3_4418_add8_4042a88d2da5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseOverrides4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseStatics {
    type Vtable = IFlyoutBaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2d795e3_85c0_4de2_bac1_5294ca011a78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flyoutowner: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseStatics2 {
    type Vtable = IFlyoutBaseStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8e913fe_2d60_4307_aad9_56b450121b58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseStatics3 {
    type Vtable = IFlyoutBaseStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba92e4f_dd16_4be4_99db_bd9d4406c0f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics3_abi(
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
pub struct IFlyoutBaseStatics5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseStatics5 {
    type Vtable = IFlyoutBaseStatics5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69edb25c_992a_542b_bcff_2f7f855523bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutBaseStatics6 {
    type Vtable = IFlyoutBaseStatics6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96d49254_c91b_5246_8b39_afc2a2c50cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics6_abi(
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
pub struct IFlyoutShowOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutShowOptions {
    type Vtable = IFlyoutShowOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57d693ad_0c74_54dd_b110_1ee43fabadd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutShowOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FlyoutShowMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FlyoutShowMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FlyoutPlacementMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FlyoutPlacementMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlyoutShowOptionsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlyoutShowOptionsFactory {
    type Vtable = IFlyoutShowOptionsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce596f61_2eb4_5b4e_af69_f9af42320eee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutShowOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeneratorPositionHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeneratorPositionHelper {
    type Vtable = IGeneratorPositionHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd40318d_7745_40d9_ab9d_abbda4a7ffea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneratorPositionHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeneratorPositionHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeneratorPositionHelperStatics {
    type Vtable = IGeneratorPositionHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad4095cd_60ec_4588_8d60_39d29097a4df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneratorPositionHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, offset: i32, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGridViewItemPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridViewItemPresenter {
    type Vtable = IGridViewItemPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x214f9010_56e2_4821_8a1c_2305709af94b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::HorizontalAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::HorizontalAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::VerticalAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::VerticalAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGridViewItemPresenterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridViewItemPresenterFactory {
    type Vtable = IGridViewItemPresenterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53c12178_63bb_4a65_a3f1_ab114cfc6ffe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGridViewItemPresenterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridViewItemPresenterStatics {
    type Vtable = IGridViewItemPresenterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe958f8c4_277e_4a72_a01e_9e1688980178);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGridViewItemTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridViewItemTemplateSettings {
    type Vtable = IGridViewItemTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e30baaf_165d_4267_a45e_1a43a75706ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IItemsChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IItemsChangedEventArgs {
    type Vtable = IItemsChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8b45568_7d10_421e_be29_81839a91de20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemsChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJumpListItemBackgroundConverter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJumpListItemBackgroundConverter {
    type Vtable = IJumpListItemBackgroundConverter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81177858_d224_410c_b16c_c5b6bb6188b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemBackgroundConverter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJumpListItemBackgroundConverterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJumpListItemBackgroundConverterStatics {
    type Vtable = IJumpListItemBackgroundConverterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20e7c3dd_6f27_4808_b0be_83e0e9b5cc45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemBackgroundConverterStatics_abi(
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
pub struct IJumpListItemForegroundConverter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJumpListItemForegroundConverter {
    type Vtable = IJumpListItemForegroundConverter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1590ed38_c504_4796_a63a_5bfc9eefaae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemForegroundConverter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJumpListItemForegroundConverterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJumpListItemForegroundConverterStatics {
    type Vtable = IJumpListItemForegroundConverterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x474e7352_210c_4673_ac6a_413f0e2c7750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemForegroundConverterStatics_abi(
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
pub struct ILayoutInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILayoutInformation {
    type Vtable = ILayoutInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5384c9b_c8cf_41b3_bf16_18c8420e72c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILayoutInformationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILayoutInformationStatics {
    type Vtable = ILayoutInformationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf06cf99_58e9_4682_8326_50caab65ed7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispatcher: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILayoutInformationStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILayoutInformationStatics2 {
    type Vtable = ILayoutInformationStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x760315b5_6d4e_4939_ac61_639863cea36b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformationStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenter {
    type Vtable = IListViewItemPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc8946bd_a3a2_4969_8174_25b5d3c28033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::HorizontalAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::HorizontalAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::VerticalAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::VerticalAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenter2 {
    type Vtable = IListViewItemPresenter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5dc5496_e122_4c57_a625_ac4b08fb2d4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ListViewItemPresenterCheckMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ListViewItemPresenterCheckMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenter3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenter3 {
    type Vtable = IListViewItemPresenter3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36620013_0390_4e30_ad97_8744404f7010);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenter4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenter4 {
    type Vtable = IListViewItemPresenter4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda600ac1_adea_5940_a18f_57582f96d99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::CornerRadius) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::CornerRadius) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenterFactory {
    type Vtable = IListViewItemPresenterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0777cfd_f7e4_4a67_9ac0_a994fcacd020);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenterStatics {
    type Vtable = IListViewItemPresenterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6504a55a_15dd_42fb_aa5d_2d8ce2e9c294);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenterStatics2 {
    type Vtable = IListViewItemPresenterStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cb3b945_d24d_42a3_9e83_a86d0618bf1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenterStatics3 {
    type Vtable = IListViewItemPresenterStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3d3d11e_fa26_4ce7_a4ed_ff948f01b7c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemPresenterStatics4 {
    type Vtable = IListViewItemPresenterStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3917159e_74a1_5e7e_a743_e45be9fb919b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IListViewItemTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IListViewItemTemplateSettings {
    type Vtable = IListViewItemTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67af84bf_8279_4686_9326_cd189f27575d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoopingSelector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoopingSelector {
    type Vtable = ILoopingSelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c9a3e04_4827_49d9_8806_093957b0fd21);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoopingSelectorItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoopingSelectorItem {
    type Vtable = ILoopingSelectorItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc69714b9_27c6_4433_9d7c_0dbfb2f4344f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoopingSelectorPanel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoopingSelectorPanel {
    type Vtable = ILoopingSelectorPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40a9ba70_1011_4778_87f7_6bfd20d6377d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoopingSelectorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoopingSelectorStatics {
    type Vtable = ILoopingSelectorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e8bafa_8c7d_4fc5_b92a_f049fb933cc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMenuFlyoutItemTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMenuFlyoutItemTemplateSettings {
    type Vtable = IMenuFlyoutItemTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56ad1809_3a16_4147_81cb_d0b35c834e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutItemTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMenuFlyoutPresenterTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMenuFlyoutPresenterTemplateSettings {
    type Vtable = IMenuFlyoutPresenterTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd68fd00d_629d_4349_ac51_b877c80983b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutPresenterTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationViewItemPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationViewItemPresenter {
    type Vtable = INavigationViewItemPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9956d3fc_4693_59cb_b6bf_37249058be96);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationViewItemPresenterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationViewItemPresenterFactory {
    type Vtable = INavigationViewItemPresenterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb062c50_4a36_52e7_9459_e89d02f3fc42);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationViewItemPresenterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationViewItemPresenterStatics {
    type Vtable = INavigationViewItemPresenterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52814604_cfc1_5ad5_a3aa_fa355be6bd76);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenterStatics_abi(
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
pub struct IOrientedVirtualizingPanel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientedVirtualizingPanel {
    type Vtable = IOrientedVirtualizingPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf077b577_39bd_46ee_bdd7_0826beed71b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientedVirtualizingPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientedVirtualizingPanelFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientedVirtualizingPanelFactory {
    type Vtable = IOrientedVirtualizingPanelFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b8eaeaf_f92f_439d_9ebf_e9919f56c94d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientedVirtualizingPanelFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPickerFlyoutBase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPickerFlyoutBase {
    type Vtable = IPickerFlyoutBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe33574ea_1076_44d1_9383_dc24ac5cff2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPickerFlyoutBaseFactory {
    type Vtable = IPickerFlyoutBaseFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ec27a53_9502_4beb_b342_00566c8f16b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPickerFlyoutBaseOverrides {
    type Vtable = IPickerFlyoutBaseOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bfc4f4a_4822_47b4_a958_77c20ba120d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPickerFlyoutBaseStatics {
    type Vtable = IPickerFlyoutBaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a4d0ac5_89ae_40e5_a7f1_bb702355adf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPivotHeaderItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPivotHeaderItem {
    type Vtable = IPivotHeaderItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594572c2_82aa_410b_9e55_fd8e2c98862d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPivotHeaderItemFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPivotHeaderItemFactory {
    type Vtable = IPivotHeaderItemFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14308b37_185b_4117_bc77_dda2eb261b99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderItemFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPivotHeaderPanel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPivotHeaderPanel {
    type Vtable = IPivotHeaderPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21484ebc_9241_4203_bd37_6c08fb096612);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPivotPanel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPivotPanel {
    type Vtable = IPivotPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad4ebe80_22a9_4ca3_9212_2773b6359ff3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopup(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopup {
    type Vtable = IPopup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62418240_e6d3_4705_a1dc_39156456ee29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopup2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopup2 {
    type Vtable = IPopup2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x376a8c4c_aac0_4b20_966a_0b9364feb4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::LightDismissOverlayMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::LightDismissOverlayMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopup3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopup3 {
    type Vtable = IPopup3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9c46915_a65c_5f68_9f54_310a1b51095f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopup4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopup4 {
    type Vtable = IPopup4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1870b836_df2f_5fc6_a5f2_748ed6ce7321);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PopupPlacementMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PopupPlacementMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PopupPlacementMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopupStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopupStatics {
    type Vtable = IPopupStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ae3bf1a_6e34_40d6_8a7f_ca822aaf59e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopupStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopupStatics2 {
    type Vtable = IPopupStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b9ae9ec_55ef_43b6_b459_12e40ffa4302);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics2_abi(
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
pub struct IPopupStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopupStatics3 {
    type Vtable = IPopupStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00789589_c580_558f_945a_7d02ee004d3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics3_abi(
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
pub struct IPopupStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopupStatics4 {
    type Vtable = IPopupStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1a42c06_8bfa_5164_8554_48bfe6bd4cc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics4_abi(
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
pub struct IProgressBarTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProgressBarTemplateSettings {
    type Vtable = IProgressBarTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fe2ea2a_e3f2_4c2b_9488_918d77d2bbe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressBarTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProgressRingTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProgressRingTemplateSettings {
    type Vtable = IProgressRingTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9b675ec_c723_42e6_83e9_9826272bdc0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressRingTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRangeBase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeBase {
    type Vtable = IRangeBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa002c1a_494e_46cf_91d4_e14a8d798675);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRangeBaseFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeBaseFactory {
    type Vtable = IRangeBaseFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x389b7c71_5220_42b2_9992_2690c1a67030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRangeBaseOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeBaseOverrides {
    type Vtable = IRangeBaseOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4291af39_7f0b_4bc2_99c4_06e7062682d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldminimum: f64, newminimum: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldmaximum: f64, newmaximum: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldvalue: f64, newvalue: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRangeBaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeBaseStatics {
    type Vtable = IRangeBaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67ef17e1_fe37_474f_9e97_3b5e0b30f2e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRangeBaseValueChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeBaseValueChangedEventArgs {
    type Vtable = IRangeBaseValueChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1921777_d5c1_4f9c_a7b0_0401b7e6dc5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseValueChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepeatButton(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepeatButton {
    type Vtable = IRepeatButton_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02200df9_021a_484a_a93b_0f31020314e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatButton_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepeatButtonStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepeatButtonStatics {
    type Vtable = IRepeatButtonStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3914ac4e_f462_4f73_8197_e8846639c682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatButtonStatics_abi(
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
pub struct IScrollBar(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScrollBar {
    type Vtable = IScrollBar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf57ae6ca_d1a6_4b90_a4e9_54df1ba8d2ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollBar_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Orientation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Orientation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ScrollingIndicatorMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ScrollingIndicatorMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScrollBarStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScrollBarStatics {
    type Vtable = IScrollBarStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45eaf38d_b814_48cf_97f2_539eb16dfd4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollBarStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScrollEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScrollEventArgs {
    type Vtable = IScrollEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc57e5168_3afe_448d_b752_2f364c75d743);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ScrollEventType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IScrollSnapPointsInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScrollSnapPointsInfo {
    type Vtable = IScrollSnapPointsInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b5d1336_e61b_4d51_be41_fd8ddc55c58c);
}
impl IScrollSnapPointsInfo {
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), orientation, alignment, &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), orientation, alignment, offset, &mut result__).from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IScrollSnapPointsInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1b5d1336-e61b-4d51-be41-fd8ddc55c58c}");
}
impl ::core::convert::From<IScrollSnapPointsInfo> for ::windows::core::IUnknown {
    fn from(value: IScrollSnapPointsInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IScrollSnapPointsInfo> for ::windows::core::IUnknown {
    fn from(value: &IScrollSnapPointsInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IScrollSnapPointsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IScrollSnapPointsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IScrollSnapPointsInfo> for ::windows::core::IInspectable {
    fn from(value: IScrollSnapPointsInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IScrollSnapPointsInfo> for ::windows::core::IInspectable {
    fn from(value: &IScrollSnapPointsInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IScrollSnapPointsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IScrollSnapPointsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollSnapPointsInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelector {
    type Vtable = ISelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe30eb3a5_b36b_42dc_8527_cd25136c083c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectorFactory {
    type Vtable = ISelectorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9be2995_d136_4600_b187_8ad56079b48a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectorItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectorItem {
    type Vtable = ISelectorItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x541c8d6c_0283_4581_b945_2a64c28a0646);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItem_abi(
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
pub struct ISelectorItemFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectorItemFactory {
    type Vtable = ISelectorItemFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9363945_c86a_4b1e_9440_1879378d5313);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItemFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectorItemStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectorItemStatics {
    type Vtable = ISelectorItemStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a353ab8_cbe9_4303_92e7_c8906e218392);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItemStatics_abi(
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
pub struct ISelectorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectorStatics {
    type Vtable = ISelectorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13300b06_bd10_4e09_bff7_71efb8bbb42b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISettingsFlyoutTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISettingsFlyoutTemplateSettings {
    type Vtable = ISettingsFlyoutTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcf14c10_cea7_43f1_9d68_57605ded69d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsFlyoutTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplitViewTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplitViewTemplateSettings {
    type Vtable = ISplitViewTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc16ab5a7_4996_4443_b199_6b6b89124eab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitViewTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IThumb(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IThumb {
    type Vtable = IThumb_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8b2b281_0d6a_45cf_b333_2402b037f099);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumb_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IThumbStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IThumbStatics {
    type Vtable = IThumbStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x955024eb_36f3_4672_a186_baaf626ac4ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbStatics_abi(
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
pub struct ITickBar(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITickBar {
    type Vtable = ITickBar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x994683fa_f1f6_487d_a5ac_c15921bfa995);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITickBar_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITickBarStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITickBarStatics {
    type Vtable = ITickBarStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c6d7e40_799d_4a54_be09_1fefc61d018e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITickBarStatics_abi(
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
pub struct IToggleButton(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToggleButton {
    type Vtable = IToggleButton_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x589877fb_0fc7_4036_9d8b_127dfa75c16d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButton_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToggleButtonFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToggleButtonFactory {
    type Vtable = IToggleButtonFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd56aa2fc_fc7f_449c_9855_7a1055d668a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToggleButtonOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToggleButtonOverrides {
    type Vtable = IToggleButtonOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd20e4c28_f18b_491a_9a45_f1a04a9369a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonOverrides_abi(
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
pub struct IToggleButtonStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToggleButtonStatics {
    type Vtable = IToggleButtonStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf1eab12_0128_4f67_9c5a_82320c445d19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonStatics_abi(
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
pub struct IToggleSwitchTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToggleSwitchTemplateSettings {
    type Vtable = IToggleSwitchTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b7bdcd_628a_4363_86e0_51d6e2e89e58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleSwitchTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToolTipTemplateSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToolTipTemplateSettings {
    type Vtable = IToolTipTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4388247_0ec4_4506_affd_afac2225b48c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolTipTemplateSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ItemsChangedEventArgs(pub ::windows::core::IInspectable);
impl ItemsChangedEventArgs {
    pub fn Action(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<GeneratorPosition> {
        let this = self;
        unsafe {
            let mut result__: GeneratorPosition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeneratorPosition>(result__)
        }
    }
    pub fn OldPosition(&self) -> ::windows::core::Result<GeneratorPosition> {
        let this = self;
        unsafe {
            let mut result__: GeneratorPosition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeneratorPosition>(result__)
        }
    }
    pub fn ItemCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn ItemUICount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ItemsChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ItemsChangedEventArgs;{e8b45568-7d10-421e-be29-81839a91de20})");
}
unsafe impl ::windows::core::Interface for ItemsChangedEventArgs {
    type Vtable = IItemsChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8b45568_7d10_421e_be29_81839a91de20);
}
impl ::windows::core::RuntimeName for ItemsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ItemsChangedEventArgs";
}
impl ::core::convert::From<ItemsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ItemsChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ItemsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ItemsChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ItemsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ItemsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ItemsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ItemsChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ItemsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ItemsChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ItemsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ItemsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ItemsChangedEventArgs {}
unsafe impl ::core::marker::Sync for ItemsChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ItemsChangedEventHandler(::windows::core::IUnknown);
impl ItemsChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ItemsChangedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = ItemsChangedEventHandler_box::<F> {
            vtable: &ItemsChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ItemsChangedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ItemsChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({178257be-a304-482f-8bf0-b9d2e39612a3})");
}
unsafe impl ::windows::core::Interface for ItemsChangedEventHandler {
    type Vtable = ItemsChangedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178257be_a304_482f_8bf0_b9d2e39612a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ItemsChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct ItemsChangedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ItemsChangedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const ItemsChangedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ItemsChangedEventArgs>) -> ::windows::core::Result<()> + 'static> ItemsChangedEventHandler_box<F> {
    const VTABLE: ItemsChangedEventHandler_abi = ItemsChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ItemsChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <ItemsChangedEventArgs as ::windows::core::Abi>::Abi as *const <ItemsChangedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct JumpListItemBackgroundConverter(pub ::windows::core::IInspectable);
impl JumpListItemBackgroundConverter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JumpListItemBackgroundConverter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Enabled(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Disabled(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDisabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "UI_Xaml_Data", feature = "UI_Xaml_Interop"))]
    pub fn Convert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::super::Interop::TypeName>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::Data::IValueConverter>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "UI_Xaml_Data", feature = "UI_Xaml_Interop"))]
    pub fn ConvertBack<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::super::Interop::TypeName>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::Data::IValueConverter>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn EnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IJumpListItemBackgroundConverterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DisabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IJumpListItemBackgroundConverterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IJumpListItemBackgroundConverterStatics<R, F: FnOnce(&IJumpListItemBackgroundConverterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JumpListItemBackgroundConverter, IJumpListItemBackgroundConverterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for JumpListItemBackgroundConverter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.JumpListItemBackgroundConverter;{81177858-d224-410c-b16c-c5b6bb6188b2})");
}
unsafe impl ::windows::core::Interface for JumpListItemBackgroundConverter {
    type Vtable = IJumpListItemBackgroundConverter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81177858_d224_410c_b16c_c5b6bb6188b2);
}
impl ::windows::core::RuntimeName for JumpListItemBackgroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.JumpListItemBackgroundConverter";
}
impl ::core::convert::From<JumpListItemBackgroundConverter> for ::windows::core::IUnknown {
    fn from(value: JumpListItemBackgroundConverter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&JumpListItemBackgroundConverter> for ::windows::core::IUnknown {
    fn from(value: &JumpListItemBackgroundConverter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<JumpListItemBackgroundConverter> for ::windows::core::IInspectable {
    fn from(value: JumpListItemBackgroundConverter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&JumpListItemBackgroundConverter> for ::windows::core::IInspectable {
    fn from(value: &JumpListItemBackgroundConverter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl ::core::convert::TryFrom<JumpListItemBackgroundConverter> for super::super::Data::IValueConverter {
    type Error = ::windows::core::Error;
    fn try_from(value: JumpListItemBackgroundConverter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl ::core::convert::TryFrom<&JumpListItemBackgroundConverter> for super::super::Data::IValueConverter {
    type Error = ::windows::core::Error;
    fn try_from(value: &JumpListItemBackgroundConverter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Data::IValueConverter> for JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Data::IValueConverter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Data::IValueConverter> for &JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Data::IValueConverter> {
        ::core::convert::TryInto::<super::super::Data::IValueConverter>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<JumpListItemBackgroundConverter> for super::super::DependencyObject {
    fn from(value: JumpListItemBackgroundConverter) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&JumpListItemBackgroundConverter> for super::super::DependencyObject {
    fn from(value: &JumpListItemBackgroundConverter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for JumpListItemBackgroundConverter {}
unsafe impl ::core::marker::Sync for JumpListItemBackgroundConverter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct JumpListItemForegroundConverter(pub ::windows::core::IInspectable);
impl JumpListItemForegroundConverter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JumpListItemForegroundConverter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Enabled(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Disabled(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDisabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "UI_Xaml_Data", feature = "UI_Xaml_Interop"))]
    pub fn Convert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::super::Interop::TypeName>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::Data::IValueConverter>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "UI_Xaml_Data", feature = "UI_Xaml_Interop"))]
    pub fn ConvertBack<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::super::Interop::TypeName>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::Data::IValueConverter>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn EnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IJumpListItemForegroundConverterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DisabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IJumpListItemForegroundConverterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IJumpListItemForegroundConverterStatics<R, F: FnOnce(&IJumpListItemForegroundConverterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JumpListItemForegroundConverter, IJumpListItemForegroundConverterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for JumpListItemForegroundConverter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.JumpListItemForegroundConverter;{1590ed38-c504-4796-a63a-5bfc9eefaae8})");
}
unsafe impl ::windows::core::Interface for JumpListItemForegroundConverter {
    type Vtable = IJumpListItemForegroundConverter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1590ed38_c504_4796_a63a_5bfc9eefaae8);
}
impl ::windows::core::RuntimeName for JumpListItemForegroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.JumpListItemForegroundConverter";
}
impl ::core::convert::From<JumpListItemForegroundConverter> for ::windows::core::IUnknown {
    fn from(value: JumpListItemForegroundConverter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&JumpListItemForegroundConverter> for ::windows::core::IUnknown {
    fn from(value: &JumpListItemForegroundConverter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<JumpListItemForegroundConverter> for ::windows::core::IInspectable {
    fn from(value: JumpListItemForegroundConverter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&JumpListItemForegroundConverter> for ::windows::core::IInspectable {
    fn from(value: &JumpListItemForegroundConverter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl ::core::convert::TryFrom<JumpListItemForegroundConverter> for super::super::Data::IValueConverter {
    type Error = ::windows::core::Error;
    fn try_from(value: JumpListItemForegroundConverter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl ::core::convert::TryFrom<&JumpListItemForegroundConverter> for super::super::Data::IValueConverter {
    type Error = ::windows::core::Error;
    fn try_from(value: &JumpListItemForegroundConverter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Data::IValueConverter> for JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Data::IValueConverter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Data::IValueConverter> for &JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Data::IValueConverter> {
        ::core::convert::TryInto::<super::super::Data::IValueConverter>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<JumpListItemForegroundConverter> for super::super::DependencyObject {
    fn from(value: JumpListItemForegroundConverter) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&JumpListItemForegroundConverter> for super::super::DependencyObject {
    fn from(value: &JumpListItemForegroundConverter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for JumpListItemForegroundConverter {}
unsafe impl ::core::marker::Sync for JumpListItemForegroundConverter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LayoutInformation(pub ::windows::core::IInspectable);
impl LayoutInformation {
    pub fn GetLayoutExceptionElement<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(dispatcher: Param0) -> ::windows::core::Result<super::super::UIElement> {
        Self::ILayoutInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dispatcher.into_param().abi(), &mut result__).from_abi::<super::super::UIElement>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetLayoutSlot<'a, Param0: ::windows::core::IntoParam<'a, super::super::FrameworkElement>>(element: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        Self::ILayoutInformationStatics(|this| unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetAvailableSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::Size> {
        Self::ILayoutInformationStatics2(|this| unsafe {
            let mut result__: super::super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Size>(result__)
        })
    }
    pub fn ILayoutInformationStatics<R, F: FnOnce(&ILayoutInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LayoutInformation, ILayoutInformationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILayoutInformationStatics2<R, F: FnOnce(&ILayoutInformationStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LayoutInformation, ILayoutInformationStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LayoutInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.LayoutInformation;{b5384c9b-c8cf-41b3-bf16-18c8420e72c9})");
}
unsafe impl ::windows::core::Interface for LayoutInformation {
    type Vtable = ILayoutInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5384c9b_c8cf_41b3_bf16_18c8420e72c9);
}
impl ::windows::core::RuntimeName for LayoutInformation {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LayoutInformation";
}
impl ::core::convert::From<LayoutInformation> for ::windows::core::IUnknown {
    fn from(value: LayoutInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LayoutInformation> for ::windows::core::IUnknown {
    fn from(value: &LayoutInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LayoutInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LayoutInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LayoutInformation> for ::windows::core::IInspectable {
    fn from(value: LayoutInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LayoutInformation> for ::windows::core::IInspectable {
    fn from(value: &LayoutInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LayoutInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LayoutInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LayoutInformation {}
unsafe impl ::core::marker::Sync for LayoutInformation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ListViewItemPresenter(pub ::windows::core::IInspectable);
impl ListViewItemPresenter {
    pub fn SelectionCheckMarkVisualEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelectionCheckMarkVisualEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckHintBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckHintBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckSelectingBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckSelectingBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn DragBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDragBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn DragForeground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDragForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PlaceholderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPlaceholderBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPointerOverBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedForeground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPointerOverBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPointerOverBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetSelectedBorderThickness<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisabledOpacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDisabledOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DragOpacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDragOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReorderHintOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetReorderHintOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterHorizontalContentAlignment(&self) -> ::windows::core::Result<super::super::HorizontalAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::super::HorizontalAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::HorizontalAlignment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetListViewItemPresenterHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterVerticalContentAlignment(&self) -> ::windows::core::Result<super::super::VerticalAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::super::VerticalAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::VerticalAlignment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetListViewItemPresenterVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterPadding(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetListViewItemPresenterPadding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PointerOverBackgroundMargin(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetPointerOverBackgroundMargin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContentMargin(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetContentMargin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPressedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPressedBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PressedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPressedBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusSecondaryBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusSecondaryBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CheckMode(&self) -> ::windows::core::Result<ListViewItemPresenterCheckMode> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__: ListViewItemPresenterCheckMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ListViewItemPresenterCheckMode>(result__)
        }
    }
    pub fn SetCheckMode(&self, value: ListViewItemPresenterCheckMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PointerOverForeground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPointerOverForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectionCheckMarkVisualEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckHintBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckSelectingBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragForegroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FocusBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PlaceholderBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedForegroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPointerOverBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPointerOverBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBorderThicknessProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DisabledOpacityProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragOpacityProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ReorderHintOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterHorizontalContentAlignmentProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterVerticalContentAlignmentProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterPaddingProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBackgroundMarginProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentMarginProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPressedBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PressedBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FocusSecondaryBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverForegroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<ListViewItemPresenter> {
        Self::IListViewItemPresenterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ListViewItemPresenter>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn RevealBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetRevealBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn RevealBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetRevealBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RevealBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetRevealBorderThickness<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RevealBackgroundShowsAboveContent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRevealBackgroundShowsAboveContent(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RevealBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RevealBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RevealBorderThicknessProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RevealBackgroundShowsAboveContentProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedDisabledBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedDisabledBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckPressedBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckDisabledBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxPointerOverBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxPressedBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxDisabledBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxSelectedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxSelectedBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxSelectedPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxSelectedPointerOverBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxSelectedPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxSelectedPressedBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxSelectedDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxSelectedDisabledBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxPointerOverBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxPressedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxPressedBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxDisabledBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxDisabledBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CheckBoxCornerRadius(&self) -> ::windows::core::Result<super::super::CornerRadius> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: super::super::CornerRadius = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::CornerRadius>(result__)
        }
    }
    pub fn SetCheckBoxCornerRadius<'a, Param0: ::windows::core::IntoParam<'a, super::super::CornerRadius>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectionIndicatorCornerRadius(&self) -> ::windows::core::Result<super::super::CornerRadius> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: super::super::CornerRadius = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::CornerRadius>(result__)
        }
    }
    pub fn SetSelectionIndicatorCornerRadius<'a, Param0: ::windows::core::IntoParam<'a, super::super::CornerRadius>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectionIndicatorVisualEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelectionIndicatorVisualEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SelectionIndicatorMode(&self) -> ::windows::core::Result<ListViewItemPresenterSelectionIndicatorMode> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ListViewItemPresenterSelectionIndicatorMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ListViewItemPresenterSelectionIndicatorMode>(result__)
        }
    }
    pub fn SetSelectionIndicatorMode(&self, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectionIndicatorBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectionIndicatorBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectionIndicatorPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectionIndicatorPointerOverBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectionIndicatorPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectionIndicatorPressedBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectionIndicatorDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectionIndicatorDisabledBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPressedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPressedBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedDisabledBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedDisabledBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedInnerBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedInnerBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).57)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPointerOverBorderBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).59)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedDisabledBackgroundProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckPressedBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckDisabledBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxPointerOverBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxPressedBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxDisabledBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxSelectedBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxSelectedPointerOverBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxSelectedPressedBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxSelectedDisabledBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxPointerOverBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxPressedBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxDisabledBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxCornerRadiusProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorCornerRadiusProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorVisualEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorPointerOverBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorPressedBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorDisabledBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPressedBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedDisabledBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedInnerBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBorderBrushProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IListViewItemPresenterStatics<R, F: FnOnce(&IListViewItemPresenterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IListViewItemPresenterStatics2<R, F: FnOnce(&IListViewItemPresenterStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IListViewItemPresenterFactory<R, F: FnOnce(&IListViewItemPresenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IListViewItemPresenterStatics3<R, F: FnOnce(&IListViewItemPresenterStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IListViewItemPresenterStatics4<R, F: FnOnce(&IListViewItemPresenterStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ListViewItemPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenter;{fc8946bd-a3a2-4969-8174-25b5d3c28033})");
}
unsafe impl ::windows::core::Interface for ListViewItemPresenter {
    type Vtable = IListViewItemPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc8946bd_a3a2_4969_8174_25b5d3c28033);
}
impl ::windows::core::RuntimeName for ListViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenter";
}
impl ::core::convert::From<ListViewItemPresenter> for ::windows::core::IUnknown {
    fn from(value: ListViewItemPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for ::windows::core::IUnknown {
    fn from(value: &ListViewItemPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ListViewItemPresenter> for ::windows::core::IInspectable {
    fn from(value: ListViewItemPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for ::windows::core::IInspectable {
    fn from(value: &ListViewItemPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ListViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: ListViewItemPresenter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ListViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &ListViewItemPresenter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ListViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: ListViewItemPresenter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ListViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &ListViewItemPresenter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ListViewItemPresenter> for super::ContentPresenter {
    fn from(value: ListViewItemPresenter) -> Self {
        ::core::convert::Into::<super::ContentPresenter>::into(&value)
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for super::ContentPresenter {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentPresenter> for ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentPresenter>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentPresenter> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentPresenter> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentPresenter>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ListViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: ListViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ListViewItemPresenter> for super::super::UIElement {
    fn from(value: ListViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for super::super::UIElement {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ListViewItemPresenter> for super::super::DependencyObject {
    fn from(value: ListViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for super::super::DependencyObject {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ListViewItemPresenter {}
unsafe impl ::core::marker::Sync for ListViewItemPresenter {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ListViewItemPresenterCheckMode(pub i32);
impl ListViewItemPresenterCheckMode {
    pub const Inline: ListViewItemPresenterCheckMode = ListViewItemPresenterCheckMode(0i32);
    pub const Overlay: ListViewItemPresenterCheckMode = ListViewItemPresenterCheckMode(1i32);
}
impl ::core::convert::From<i32> for ListViewItemPresenterCheckMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ListViewItemPresenterCheckMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ListViewItemPresenterCheckMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenterCheckMode;i4)");
}
impl ::windows::core::DefaultType for ListViewItemPresenterCheckMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ListViewItemPresenterSelectionIndicatorMode(pub i32);
impl ListViewItemPresenterSelectionIndicatorMode {
    pub const Inline: ListViewItemPresenterSelectionIndicatorMode = ListViewItemPresenterSelectionIndicatorMode(0i32);
    pub const Overlay: ListViewItemPresenterSelectionIndicatorMode = ListViewItemPresenterSelectionIndicatorMode(1i32);
}
impl ::core::convert::From<i32> for ListViewItemPresenterSelectionIndicatorMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ListViewItemPresenterSelectionIndicatorMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ListViewItemPresenterSelectionIndicatorMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenterSelectionIndicatorMode;i4)");
}
impl ::windows::core::DefaultType for ListViewItemPresenterSelectionIndicatorMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ListViewItemTemplateSettings(pub ::windows::core::IInspectable);
impl ListViewItemTemplateSettings {
    pub fn DragItemsCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ListViewItemTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ListViewItemTemplateSettings;{67af84bf-8279-4686-9326-cd189f27575d})");
}
unsafe impl ::windows::core::Interface for ListViewItemTemplateSettings {
    type Vtable = IListViewItemTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67af84bf_8279_4686_9326_cd189f27575d);
}
impl ::windows::core::RuntimeName for ListViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ListViewItemTemplateSettings";
}
impl ::core::convert::From<ListViewItemTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: ListViewItemTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ListViewItemTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &ListViewItemTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ListViewItemTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: ListViewItemTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ListViewItemTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &ListViewItemTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ListViewItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: ListViewItemTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ListViewItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ListViewItemTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ListViewItemTemplateSettings {}
unsafe impl ::core::marker::Sync for ListViewItemTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoopingSelector(pub ::windows::core::IInspectable);
impl LoopingSelector {
    pub fn ShouldLoop(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldLoop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetItems<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetSelectedItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ItemWidth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetItemWidth(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ItemHeight(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetItemHeight(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ItemTemplate(&self) -> ::windows::core::Result<super::super::DataTemplate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DataTemplate>(result__)
        }
    }
    pub fn SetItemTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::DataTemplate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::SelectionChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ShouldLoopProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ItemsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedIndexProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedItemProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ItemWidthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ItemHeightProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ItemTemplateProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ILoopingSelectorStatics<R, F: FnOnce(&ILoopingSelectorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoopingSelector, ILoopingSelectorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LoopingSelector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelector;{4c9a3e04-4827-49d9-8806-093957b0fd21})");
}
unsafe impl ::windows::core::Interface for LoopingSelector {
    type Vtable = ILoopingSelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c9a3e04_4827_49d9_8806_093957b0fd21);
}
impl ::windows::core::RuntimeName for LoopingSelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelector";
}
impl ::core::convert::From<LoopingSelector> for ::windows::core::IUnknown {
    fn from(value: LoopingSelector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoopingSelector> for ::windows::core::IUnknown {
    fn from(value: &LoopingSelector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoopingSelector> for ::windows::core::IInspectable {
    fn from(value: LoopingSelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoopingSelector> for ::windows::core::IInspectable {
    fn from(value: &LoopingSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelector> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: LoopingSelector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelector> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoopingSelector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelector> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: LoopingSelector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelector> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoopingSelector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<LoopingSelector> for super::Control {
    fn from(value: LoopingSelector) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelector> for super::Control {
    fn from(value: &LoopingSelector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelector> for super::super::FrameworkElement {
    fn from(value: LoopingSelector) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelector> for super::super::FrameworkElement {
    fn from(value: &LoopingSelector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelector> for super::super::UIElement {
    fn from(value: LoopingSelector) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelector> for super::super::UIElement {
    fn from(value: &LoopingSelector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelector> for super::super::DependencyObject {
    fn from(value: LoopingSelector) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelector> for super::super::DependencyObject {
    fn from(value: &LoopingSelector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LoopingSelector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LoopingSelector {}
unsafe impl ::core::marker::Sync for LoopingSelector {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoopingSelectorItem(pub ::windows::core::IInspectable);
impl LoopingSelectorItem {}
unsafe impl ::windows::core::RuntimeType for LoopingSelectorItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelectorItem;{c69714b9-27c6-4433-9d7c-0dbfb2f4344f})");
}
unsafe impl ::windows::core::Interface for LoopingSelectorItem {
    type Vtable = ILoopingSelectorItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc69714b9_27c6_4433_9d7c_0dbfb2f4344f);
}
impl ::windows::core::RuntimeName for LoopingSelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelectorItem";
}
impl ::core::convert::From<LoopingSelectorItem> for ::windows::core::IUnknown {
    fn from(value: LoopingSelectorItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for ::windows::core::IUnknown {
    fn from(value: &LoopingSelectorItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoopingSelectorItem> for ::windows::core::IInspectable {
    fn from(value: LoopingSelectorItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for ::windows::core::IInspectable {
    fn from(value: &LoopingSelectorItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelectorItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: LoopingSelectorItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelectorItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoopingSelectorItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelectorItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: LoopingSelectorItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelectorItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoopingSelectorItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::ContentControl {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::Into::<super::ContentControl>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::ContentControl {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::Control {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::Control {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::super::FrameworkElement {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::super::FrameworkElement {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::super::UIElement {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::super::UIElement {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::super::DependencyObject {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::super::DependencyObject {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LoopingSelectorItem {}
unsafe impl ::core::marker::Sync for LoopingSelectorItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoopingSelectorPanel(pub ::windows::core::IInspectable);
impl LoopingSelectorPanel {
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), orientation, alignment, &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), orientation, alignment, offset, &mut result__).from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LoopingSelectorPanel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelectorPanel;{40a9ba70-1011-4778-87f7-6bfd20d6377d})");
}
unsafe impl ::windows::core::Interface for LoopingSelectorPanel {
    type Vtable = ILoopingSelectorPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40a9ba70_1011_4778_87f7_6bfd20d6377d);
}
impl ::windows::core::RuntimeName for LoopingSelectorPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelectorPanel";
}
impl ::core::convert::From<LoopingSelectorPanel> for ::windows::core::IUnknown {
    fn from(value: LoopingSelectorPanel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for ::windows::core::IUnknown {
    fn from(value: &LoopingSelectorPanel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for ::windows::core::IInspectable {
    fn from(value: LoopingSelectorPanel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for ::windows::core::IInspectable {
    fn from(value: &LoopingSelectorPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<LoopingSelectorPanel> for IScrollSnapPointsInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: LoopingSelectorPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoopingSelectorPanel> for IScrollSnapPointsInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoopingSelectorPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IScrollSnapPointsInfo> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, IScrollSnapPointsInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IScrollSnapPointsInfo> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, IScrollSnapPointsInfo> {
        ::core::convert::TryInto::<IScrollSnapPointsInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelectorPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: LoopingSelectorPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelectorPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoopingSelectorPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelectorPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: LoopingSelectorPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelectorPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoopingSelectorPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::Canvas {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::Into::<super::Canvas>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::Canvas {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Canvas> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Canvas> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Canvas>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Canvas> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Canvas> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Canvas>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::Panel {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::Into::<super::Panel>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::Panel {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::super::FrameworkElement {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::super::FrameworkElement {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::super::UIElement {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::super::UIElement {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::super::DependencyObject {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::super::DependencyObject {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LoopingSelectorPanel {}
unsafe impl ::core::marker::Sync for LoopingSelectorPanel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MenuFlyoutItemTemplateSettings(pub ::windows::core::IInspectable);
impl MenuFlyoutItemTemplateSettings {
    pub fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MenuFlyoutItemTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.MenuFlyoutItemTemplateSettings;{56ad1809-3a16-4147-81cb-d0b35c834e0f})");
}
unsafe impl ::windows::core::Interface for MenuFlyoutItemTemplateSettings {
    type Vtable = IMenuFlyoutItemTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56ad1809_3a16_4147_81cb_d0b35c834e0f);
}
impl ::windows::core::RuntimeName for MenuFlyoutItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.MenuFlyoutItemTemplateSettings";
}
impl ::core::convert::From<MenuFlyoutItemTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: MenuFlyoutItemTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MenuFlyoutItemTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &MenuFlyoutItemTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MenuFlyoutItemTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: MenuFlyoutItemTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MenuFlyoutItemTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &MenuFlyoutItemTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MenuFlyoutItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: MenuFlyoutItemTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&MenuFlyoutItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: &MenuFlyoutItemTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for MenuFlyoutItemTemplateSettings {}
unsafe impl ::core::marker::Sync for MenuFlyoutItemTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MenuFlyoutPresenterTemplateSettings(pub ::windows::core::IInspectable);
impl MenuFlyoutPresenterTemplateSettings {
    pub fn FlyoutContentMinWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MenuFlyoutPresenterTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.MenuFlyoutPresenterTemplateSettings;{d68fd00d-629d-4349-ac51-b877c80983b8})");
}
unsafe impl ::windows::core::Interface for MenuFlyoutPresenterTemplateSettings {
    type Vtable = IMenuFlyoutPresenterTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd68fd00d_629d_4349_ac51_b877c80983b8);
}
impl ::windows::core::RuntimeName for MenuFlyoutPresenterTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.MenuFlyoutPresenterTemplateSettings";
}
impl ::core::convert::From<MenuFlyoutPresenterTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: MenuFlyoutPresenterTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MenuFlyoutPresenterTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &MenuFlyoutPresenterTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MenuFlyoutPresenterTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: MenuFlyoutPresenterTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MenuFlyoutPresenterTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &MenuFlyoutPresenterTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MenuFlyoutPresenterTemplateSettings> for super::super::DependencyObject {
    fn from(value: MenuFlyoutPresenterTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&MenuFlyoutPresenterTemplateSettings> for super::super::DependencyObject {
    fn from(value: &MenuFlyoutPresenterTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for MenuFlyoutPresenterTemplateSettings {}
unsafe impl ::core::marker::Sync for MenuFlyoutPresenterTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigationViewItemPresenter(pub ::windows::core::IInspectable);
impl NavigationViewItemPresenter {
    pub fn Icon(&self) -> ::windows::core::Result<super::IconElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IconElement>(result__)
        }
    }
    pub fn SetIcon<'a, Param0: ::windows::core::IntoParam<'a, super::IconElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IconProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::INavigationViewItemPresenterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<NavigationViewItemPresenter> {
        Self::INavigationViewItemPresenterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<NavigationViewItemPresenter>(result__)
        })
    }
    pub fn INavigationViewItemPresenterStatics<R, F: FnOnce(&INavigationViewItemPresenterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NavigationViewItemPresenter, INavigationViewItemPresenterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn INavigationViewItemPresenterFactory<R, F: FnOnce(&INavigationViewItemPresenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NavigationViewItemPresenter, INavigationViewItemPresenterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationViewItemPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter;{9956d3fc-4693-59cb-b6bf-37249058be96})");
}
unsafe impl ::windows::core::Interface for NavigationViewItemPresenter {
    type Vtable = INavigationViewItemPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9956d3fc_4693_59cb_b6bf_37249058be96);
}
impl ::windows::core::RuntimeName for NavigationViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter";
}
impl ::core::convert::From<NavigationViewItemPresenter> for ::windows::core::IUnknown {
    fn from(value: NavigationViewItemPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for ::windows::core::IUnknown {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for ::windows::core::IInspectable {
    fn from(value: NavigationViewItemPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for ::windows::core::IInspectable {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<NavigationViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: NavigationViewItemPresenter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&NavigationViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &NavigationViewItemPresenter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<NavigationViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: NavigationViewItemPresenter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&NavigationViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &NavigationViewItemPresenter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::ContentControl {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::Into::<super::ContentControl>::into(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::ContentControl {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::Control {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::Control {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::super::UIElement {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::super::UIElement {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::super::DependencyObject {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::super::DependencyObject {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for NavigationViewItemPresenter {}
unsafe impl ::core::marker::Sync for NavigationViewItemPresenter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OrientedVirtualizingPanel(pub ::windows::core::IInspectable);
impl OrientedVirtualizingPanel {
    pub fn CanVerticallyScroll(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanVerticallyScroll(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CanHorizontallyScroll(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanHorizontallyScroll(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ExtentWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExtentHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ViewportWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ViewportHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn HorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ScrollOwner(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetScrollOwner<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn LineUp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LineDown(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LineLeft(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LineRight(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PageUp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PageDown(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PageLeft(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PageRight(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MouseWheelUp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MouseWheelDown(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MouseWheelLeft(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MouseWheelRight(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetHorizontalOffset(&self, offset: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), offset).ok() }
    }
    pub fn SetVerticalOffset(&self, offset: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), offset).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MakeVisible<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>>(&self, visual: Param0, rectangle: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), visual.into_param().abi(), rectangle.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), orientation, alignment, &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), orientation, alignment, offset, &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetInsertionIndexes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, position: Param0, first: &mut i32, second: &mut i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IInsertionPanel>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), first, second).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for OrientedVirtualizingPanel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.OrientedVirtualizingPanel;{f077b577-39bd-46ee-bdd7-0826beed71b8})");
}
unsafe impl ::windows::core::Interface for OrientedVirtualizingPanel {
    type Vtable = IOrientedVirtualizingPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf077b577_39bd_46ee_bdd7_0826beed71b8);
}
impl ::windows::core::RuntimeName for OrientedVirtualizingPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.OrientedVirtualizingPanel";
}
impl ::core::convert::From<OrientedVirtualizingPanel> for ::windows::core::IUnknown {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for ::windows::core::IUnknown {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for ::windows::core::IInspectable {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for ::windows::core::IInspectable {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<OrientedVirtualizingPanel> for IScrollSnapPointsInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: OrientedVirtualizingPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OrientedVirtualizingPanel> for IScrollSnapPointsInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &OrientedVirtualizingPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IScrollSnapPointsInfo> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, IScrollSnapPointsInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IScrollSnapPointsInfo> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, IScrollSnapPointsInfo> {
        ::core::convert::TryInto::<IScrollSnapPointsInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<OrientedVirtualizingPanel> for super::IInsertionPanel {
    type Error = ::windows::core::Error;
    fn try_from(value: OrientedVirtualizingPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OrientedVirtualizingPanel> for super::IInsertionPanel {
    type Error = ::windows::core::Error;
    fn try_from(value: &OrientedVirtualizingPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IInsertionPanel> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::IInsertionPanel> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IInsertionPanel> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::IInsertionPanel> {
        ::core::convert::TryInto::<super::IInsertionPanel>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<OrientedVirtualizingPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: OrientedVirtualizingPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&OrientedVirtualizingPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &OrientedVirtualizingPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<OrientedVirtualizingPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: OrientedVirtualizingPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&OrientedVirtualizingPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &OrientedVirtualizingPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::VirtualizingPanel {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::Into::<super::VirtualizingPanel>::into(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::VirtualizingPanel {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::VirtualizingPanel> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::VirtualizingPanel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::VirtualizingPanel>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::VirtualizingPanel> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::VirtualizingPanel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::VirtualizingPanel>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::Panel {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::Into::<super::Panel>::into(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::Panel {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::super::FrameworkElement {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::super::FrameworkElement {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::super::UIElement {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::super::UIElement {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::super::DependencyObject {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::super::DependencyObject {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for OrientedVirtualizingPanel {}
unsafe impl ::core::marker::Sync for OrientedVirtualizingPanel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PickerFlyoutBase(pub ::windows::core::IInspectable);
impl PickerFlyoutBase {
    pub fn TitleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPickerFlyoutBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPickerFlyoutBaseStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IPickerFlyoutBaseStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn IPickerFlyoutBaseStatics<R, F: FnOnce(&IPickerFlyoutBaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PickerFlyoutBase, IPickerFlyoutBaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PickerFlyoutBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.PickerFlyoutBase;{e33574ea-1076-44d1-9383-dc24ac5cff2a})");
}
unsafe impl ::windows::core::Interface for PickerFlyoutBase {
    type Vtable = IPickerFlyoutBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe33574ea_1076_44d1_9383_dc24ac5cff2a);
}
impl ::windows::core::RuntimeName for PickerFlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PickerFlyoutBase";
}
impl ::core::convert::From<PickerFlyoutBase> for ::windows::core::IUnknown {
    fn from(value: PickerFlyoutBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PickerFlyoutBase> for ::windows::core::IUnknown {
    fn from(value: &PickerFlyoutBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PickerFlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PickerFlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PickerFlyoutBase> for ::windows::core::IInspectable {
    fn from(value: PickerFlyoutBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PickerFlyoutBase> for ::windows::core::IInspectable {
    fn from(value: &PickerFlyoutBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PickerFlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PickerFlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PickerFlyoutBase> for FlyoutBase {
    fn from(value: PickerFlyoutBase) -> Self {
        ::core::convert::Into::<FlyoutBase>::into(&value)
    }
}
impl ::core::convert::From<&PickerFlyoutBase> for FlyoutBase {
    fn from(value: &PickerFlyoutBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, FlyoutBase> for PickerFlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, FlyoutBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<FlyoutBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, FlyoutBase> for &PickerFlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, FlyoutBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<FlyoutBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PickerFlyoutBase> for super::super::DependencyObject {
    fn from(value: PickerFlyoutBase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PickerFlyoutBase> for super::super::DependencyObject {
    fn from(value: &PickerFlyoutBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PickerFlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PickerFlyoutBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PickerFlyoutBase {}
unsafe impl ::core::marker::Sync for PickerFlyoutBase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PivotHeaderItem(pub ::windows::core::IInspectable);
impl PivotHeaderItem {
    pub fn new() -> ::windows::core::Result<PivotHeaderItem> {
        Self::IPivotHeaderItemFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<PivotHeaderItem>(result__)
        })
    }
    pub fn IPivotHeaderItemFactory<R, F: FnOnce(&IPivotHeaderItemFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PivotHeaderItem, IPivotHeaderItemFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PivotHeaderItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.PivotHeaderItem;{594572c2-82aa-410b-9e55-fd8e2c98862d})");
}
unsafe impl ::windows::core::Interface for PivotHeaderItem {
    type Vtable = IPivotHeaderItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594572c2_82aa_410b_9e55_fd8e2c98862d);
}
impl ::windows::core::RuntimeName for PivotHeaderItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotHeaderItem";
}
impl ::core::convert::From<PivotHeaderItem> for ::windows::core::IUnknown {
    fn from(value: PivotHeaderItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PivotHeaderItem> for ::windows::core::IUnknown {
    fn from(value: &PivotHeaderItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PivotHeaderItem> for ::windows::core::IInspectable {
    fn from(value: PivotHeaderItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PivotHeaderItem> for ::windows::core::IInspectable {
    fn from(value: &PivotHeaderItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotHeaderItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: PivotHeaderItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotHeaderItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &PivotHeaderItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotHeaderItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: PivotHeaderItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotHeaderItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &PivotHeaderItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::ContentControl {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::Into::<super::ContentControl>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::ContentControl {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for &PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::Control {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::Control {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::super::FrameworkElement {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::super::FrameworkElement {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::super::UIElement {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::super::UIElement {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::super::DependencyObject {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::super::DependencyObject {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PivotHeaderItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PivotHeaderItem {}
unsafe impl ::core::marker::Sync for PivotHeaderItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PivotHeaderPanel(pub ::windows::core::IInspectable);
impl PivotHeaderPanel {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PivotHeaderPanel, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PivotHeaderPanel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.PivotHeaderPanel;{21484ebc-9241-4203-bd37-6c08fb096612})");
}
unsafe impl ::windows::core::Interface for PivotHeaderPanel {
    type Vtable = IPivotHeaderPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21484ebc_9241_4203_bd37_6c08fb096612);
}
impl ::windows::core::RuntimeName for PivotHeaderPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotHeaderPanel";
}
impl ::core::convert::From<PivotHeaderPanel> for ::windows::core::IUnknown {
    fn from(value: PivotHeaderPanel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for ::windows::core::IUnknown {
    fn from(value: &PivotHeaderPanel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PivotHeaderPanel> for ::windows::core::IInspectable {
    fn from(value: PivotHeaderPanel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for ::windows::core::IInspectable {
    fn from(value: &PivotHeaderPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotHeaderPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: PivotHeaderPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotHeaderPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &PivotHeaderPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotHeaderPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: PivotHeaderPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotHeaderPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &PivotHeaderPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::Canvas {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::Into::<super::Canvas>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::Canvas {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Canvas> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Canvas> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Canvas>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Canvas> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Canvas> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Canvas>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::Panel {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::Into::<super::Panel>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::Panel {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::super::FrameworkElement {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::super::FrameworkElement {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::super::UIElement {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::super::UIElement {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::super::DependencyObject {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::super::DependencyObject {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PivotHeaderPanel {}
unsafe impl ::core::marker::Sync for PivotHeaderPanel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PivotPanel(pub ::windows::core::IInspectable);
impl PivotPanel {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PivotPanel, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), orientation, alignment, &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), orientation, alignment, offset, &mut result__).from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PivotPanel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.PivotPanel;{ad4ebe80-22a9-4ca3-9212-2773b6359ff3})");
}
unsafe impl ::windows::core::Interface for PivotPanel {
    type Vtable = IPivotPanel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad4ebe80_22a9_4ca3_9212_2773b6359ff3);
}
impl ::windows::core::RuntimeName for PivotPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotPanel";
}
impl ::core::convert::From<PivotPanel> for ::windows::core::IUnknown {
    fn from(value: PivotPanel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PivotPanel> for ::windows::core::IUnknown {
    fn from(value: &PivotPanel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PivotPanel> for ::windows::core::IInspectable {
    fn from(value: PivotPanel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PivotPanel> for ::windows::core::IInspectable {
    fn from(value: &PivotPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PivotPanel> for IScrollSnapPointsInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: PivotPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PivotPanel> for IScrollSnapPointsInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &PivotPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IScrollSnapPointsInfo> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, IScrollSnapPointsInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IScrollSnapPointsInfo> for &PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, IScrollSnapPointsInfo> {
        ::core::convert::TryInto::<IScrollSnapPointsInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: PivotPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &PivotPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: PivotPanel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &PivotPanel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<PivotPanel> for super::Panel {
    fn from(value: PivotPanel) -> Self {
        ::core::convert::Into::<super::Panel>::into(&value)
    }
}
impl ::core::convert::From<&PivotPanel> for super::Panel {
    fn from(value: &PivotPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Panel> for &PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Panel> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Panel>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotPanel> for super::super::FrameworkElement {
    fn from(value: PivotPanel) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&PivotPanel> for super::super::FrameworkElement {
    fn from(value: &PivotPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotPanel> for super::super::UIElement {
    fn from(value: PivotPanel) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&PivotPanel> for super::super::UIElement {
    fn from(value: &PivotPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PivotPanel> for super::super::DependencyObject {
    fn from(value: PivotPanel) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PivotPanel> for super::super::DependencyObject {
    fn from(value: &PivotPanel) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PivotPanel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PivotPanel {}
unsafe impl ::core::marker::Sync for PivotPanel {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlacementMode(pub i32);
impl PlacementMode {
    pub const Bottom: PlacementMode = PlacementMode(2i32);
    pub const Left: PlacementMode = PlacementMode(9i32);
    pub const Mouse: PlacementMode = PlacementMode(7i32);
    pub const Right: PlacementMode = PlacementMode(4i32);
    pub const Top: PlacementMode = PlacementMode(10i32);
}
impl ::core::convert::From<i32> for PlacementMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.PlacementMode;i4)");
}
impl ::windows::core::DefaultType for PlacementMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Popup(pub ::windows::core::IInspectable);
impl Popup {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Popup, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Child(&self) -> ::windows::core::Result<super::super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UIElement>(result__)
        }
    }
    pub fn SetChild<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn VerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub fn ChildTransitions(&self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Animation::TransitionCollection>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub fn SetChildTransitions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Animation::TransitionCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsLightDismissEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLightDismissEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Opened<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ChildProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsOpenProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn HorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn VerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ChildTransitionsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsLightDismissEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn LightDismissOverlayMode(&self) -> ::windows::core::Result<super::LightDismissOverlayMode> {
        let this = &::windows::core::Interface::cast::<IPopup2>(self)?;
        unsafe {
            let mut result__: super::LightDismissOverlayMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::LightDismissOverlayMode>(result__)
        }
    }
    pub fn SetLightDismissOverlayMode(&self, value: super::LightDismissOverlayMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPopup2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LightDismissOverlayModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ShouldConstrainToRootBounds(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPopup3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldConstrainToRootBounds(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPopup3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsConstrainedToRootBounds(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPopup3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ShouldConstrainToRootBoundsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PlacementTarget(&self) -> ::windows::core::Result<super::super::FrameworkElement> {
        let this = &::windows::core::Interface::cast::<IPopup4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::FrameworkElement>(result__)
        }
    }
    pub fn SetPlacementTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::FrameworkElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPopup4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DesiredPlacement(&self) -> ::windows::core::Result<PopupPlacementMode> {
        let this = &::windows::core::Interface::cast::<IPopup4>(self)?;
        unsafe {
            let mut result__: PopupPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PopupPlacementMode>(result__)
        }
    }
    pub fn SetDesiredPlacement(&self, value: PopupPlacementMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPopup4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ActualPlacement(&self) -> ::windows::core::Result<PopupPlacementMode> {
        let this = &::windows::core::Interface::cast::<IPopup4>(self)?;
        unsafe {
            let mut result__: PopupPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PopupPlacementMode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ActualPlacementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPopup4>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActualPlacementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPopup4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn PlacementTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DesiredPlacementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPopupStatics<R, F: FnOnce(&IPopupStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Popup, IPopupStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPopupStatics2<R, F: FnOnce(&IPopupStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Popup, IPopupStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPopupStatics3<R, F: FnOnce(&IPopupStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Popup, IPopupStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPopupStatics4<R, F: FnOnce(&IPopupStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Popup, IPopupStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Popup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.Popup;{62418240-e6d3-4705-a1dc-39156456ee29})");
}
unsafe impl ::windows::core::Interface for Popup {
    type Vtable = IPopup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62418240_e6d3_4705_a1dc_39156456ee29);
}
impl ::windows::core::RuntimeName for Popup {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Popup";
}
impl ::core::convert::From<Popup> for ::windows::core::IUnknown {
    fn from(value: Popup) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Popup> for ::windows::core::IUnknown {
    fn from(value: &Popup) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Popup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Popup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Popup> for ::windows::core::IInspectable {
    fn from(value: Popup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Popup> for ::windows::core::IInspectable {
    fn from(value: &Popup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Popup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Popup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Popup> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Popup) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Popup> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Popup) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Popup> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Popup) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Popup> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Popup) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Popup> for super::super::FrameworkElement {
    fn from(value: Popup) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Popup> for super::super::FrameworkElement {
    fn from(value: &Popup) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Popup> for super::super::UIElement {
    fn from(value: Popup) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Popup> for super::super::UIElement {
    fn from(value: &Popup) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Popup> for super::super::DependencyObject {
    fn from(value: Popup) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Popup> for super::super::DependencyObject {
    fn from(value: &Popup) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Popup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Popup {}
unsafe impl ::core::marker::Sync for Popup {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PopupPlacementMode(pub i32);
impl PopupPlacementMode {
    pub const Auto: PopupPlacementMode = PopupPlacementMode(0i32);
    pub const Top: PopupPlacementMode = PopupPlacementMode(1i32);
    pub const Bottom: PopupPlacementMode = PopupPlacementMode(2i32);
    pub const Left: PopupPlacementMode = PopupPlacementMode(3i32);
    pub const Right: PopupPlacementMode = PopupPlacementMode(4i32);
    pub const TopEdgeAlignedLeft: PopupPlacementMode = PopupPlacementMode(5i32);
    pub const TopEdgeAlignedRight: PopupPlacementMode = PopupPlacementMode(6i32);
    pub const BottomEdgeAlignedLeft: PopupPlacementMode = PopupPlacementMode(7i32);
    pub const BottomEdgeAlignedRight: PopupPlacementMode = PopupPlacementMode(8i32);
    pub const LeftEdgeAlignedTop: PopupPlacementMode = PopupPlacementMode(9i32);
    pub const LeftEdgeAlignedBottom: PopupPlacementMode = PopupPlacementMode(10i32);
    pub const RightEdgeAlignedTop: PopupPlacementMode = PopupPlacementMode(11i32);
    pub const RightEdgeAlignedBottom: PopupPlacementMode = PopupPlacementMode(12i32);
}
impl ::core::convert::From<i32> for PopupPlacementMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PopupPlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PopupPlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.PopupPlacementMode;i4)");
}
impl ::windows::core::DefaultType for PopupPlacementMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProgressBarTemplateSettings(pub ::windows::core::IInspectable);
impl ProgressBarTemplateSettings {
    pub fn EllipseDiameter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn EllipseOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn EllipseAnimationWellPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn EllipseAnimationEndPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ContainerAnimationStartPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ContainerAnimationEndPosition(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn IndicatorLengthDelta(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProgressBarTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ProgressBarTemplateSettings;{3fe2ea2a-e3f2-4c2b-9488-918d77d2bbe4})");
}
unsafe impl ::windows::core::Interface for ProgressBarTemplateSettings {
    type Vtable = IProgressBarTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fe2ea2a_e3f2_4c2b_9488_918d77d2bbe4);
}
impl ::windows::core::RuntimeName for ProgressBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ProgressBarTemplateSettings";
}
impl ::core::convert::From<ProgressBarTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: ProgressBarTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProgressBarTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &ProgressBarTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProgressBarTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: ProgressBarTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProgressBarTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &ProgressBarTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ProgressBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: ProgressBarTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ProgressBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ProgressBarTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ProgressBarTemplateSettings {}
unsafe impl ::core::marker::Sync for ProgressBarTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProgressRingTemplateSettings(pub ::windows::core::IInspectable);
impl ProgressRingTemplateSettings {
    pub fn EllipseDiameter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn EllipseOffset(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn MaxSideLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProgressRingTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ProgressRingTemplateSettings;{b9b675ec-c723-42e6-83e9-9826272bdc0e})");
}
unsafe impl ::windows::core::Interface for ProgressRingTemplateSettings {
    type Vtable = IProgressRingTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9b675ec_c723_42e6_83e9_9826272bdc0e);
}
impl ::windows::core::RuntimeName for ProgressRingTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ProgressRingTemplateSettings";
}
impl ::core::convert::From<ProgressRingTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: ProgressRingTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProgressRingTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &ProgressRingTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProgressRingTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: ProgressRingTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProgressRingTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &ProgressRingTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ProgressRingTemplateSettings> for super::super::DependencyObject {
    fn from(value: ProgressRingTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ProgressRingTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ProgressRingTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ProgressRingTemplateSettings {}
unsafe impl ::core::marker::Sync for ProgressRingTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RangeBase(pub ::windows::core::IInspectable);
impl RangeBase {
    pub fn Minimum(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetMinimum(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Maximum(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetMaximum(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SmallChange(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetSmallChange(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LargeChange(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetLargeChange(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ValueChanged<'a, Param0: ::windows::core::IntoParam<'a, RangeBaseValueChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn MinimumProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MaximumProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SmallChangeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn LargeChangeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRangeBaseStatics<R, F: FnOnce(&IRangeBaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RangeBase, IRangeBaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RangeBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.RangeBase;{fa002c1a-494e-46cf-91d4-e14a8d798675})");
}
unsafe impl ::windows::core::Interface for RangeBase {
    type Vtable = IRangeBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa002c1a_494e_46cf_91d4_e14a8d798675);
}
impl ::windows::core::RuntimeName for RangeBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RangeBase";
}
impl ::core::convert::From<RangeBase> for ::windows::core::IUnknown {
    fn from(value: RangeBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RangeBase> for ::windows::core::IUnknown {
    fn from(value: &RangeBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RangeBase> for ::windows::core::IInspectable {
    fn from(value: RangeBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RangeBase> for ::windows::core::IInspectable {
    fn from(value: &RangeBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RangeBase> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: RangeBase) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RangeBase> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &RangeBase) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RangeBase> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: RangeBase) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RangeBase> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &RangeBase) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<RangeBase> for super::Control {
    fn from(value: RangeBase) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&RangeBase> for super::Control {
    fn from(value: &RangeBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RangeBase> for super::super::FrameworkElement {
    fn from(value: RangeBase) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&RangeBase> for super::super::FrameworkElement {
    fn from(value: &RangeBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RangeBase> for super::super::UIElement {
    fn from(value: RangeBase) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&RangeBase> for super::super::UIElement {
    fn from(value: &RangeBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RangeBase> for super::super::DependencyObject {
    fn from(value: RangeBase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RangeBase> for super::super::DependencyObject {
    fn from(value: &RangeBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &RangeBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RangeBase {}
unsafe impl ::core::marker::Sync for RangeBase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RangeBaseValueChangedEventArgs(pub ::windows::core::IInspectable);
impl RangeBaseValueChangedEventArgs {
    pub fn OldValue(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn NewValue(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RangeBaseValueChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs;{a1921777-d5c1-4f9c-a7b0-0401b7e6dc5c})");
}
unsafe impl ::windows::core::Interface for RangeBaseValueChangedEventArgs {
    type Vtable = IRangeBaseValueChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1921777_d5c1_4f9c_a7b0_0401b7e6dc5c);
}
impl ::windows::core::RuntimeName for RangeBaseValueChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs";
}
impl ::core::convert::From<RangeBaseValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RangeBaseValueChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RangeBaseValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RangeBaseValueChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RangeBaseValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RangeBaseValueChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RangeBaseValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RangeBaseValueChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RangeBaseValueChangedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: RangeBaseValueChangedEventArgs) -> Self {
        ::core::convert::Into::<super::super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&RangeBaseValueChangedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &RangeBaseValueChangedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for &RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RangeBaseValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for RangeBaseValueChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RangeBaseValueChangedEventHandler(::windows::core::IUnknown);
impl RangeBaseValueChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RangeBaseValueChangedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = RangeBaseValueChangedEventHandler_box::<F> {
            vtable: &RangeBaseValueChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, RangeBaseValueChangedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RangeBaseValueChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({e3906fd9-4d1b-4ac8-a43c-c3b908742799})");
}
unsafe impl ::windows::core::Interface for RangeBaseValueChangedEventHandler {
    type Vtable = RangeBaseValueChangedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3906fd9_4d1b_4ac8_a43c_c3b908742799);
}
#[repr(C)]
#[doc(hidden)]
pub struct RangeBaseValueChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct RangeBaseValueChangedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RangeBaseValueChangedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const RangeBaseValueChangedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RangeBaseValueChangedEventArgs>) -> ::windows::core::Result<()> + 'static> RangeBaseValueChangedEventHandler_box<F> {
    const VTABLE: RangeBaseValueChangedEventHandler_abi = RangeBaseValueChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RangeBaseValueChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <RangeBaseValueChangedEventArgs as ::windows::core::Abi>::Abi as *const <RangeBaseValueChangedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RepeatButton(pub ::windows::core::IInspectable);
impl RepeatButton {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepeatButton, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Delay(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetDelay(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Interval(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetInterval(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DelayProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepeatButtonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IntervalProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepeatButtonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRepeatButtonStatics<R, F: FnOnce(&IRepeatButtonStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepeatButton, IRepeatButtonStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RepeatButton {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.RepeatButton;{02200df9-021a-484a-a93b-0f31020314e5})");
}
unsafe impl ::windows::core::Interface for RepeatButton {
    type Vtable = IRepeatButton_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02200df9_021a_484a_a93b_0f31020314e5);
}
impl ::windows::core::RuntimeName for RepeatButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RepeatButton";
}
impl ::core::convert::From<RepeatButton> for ::windows::core::IUnknown {
    fn from(value: RepeatButton) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RepeatButton> for ::windows::core::IUnknown {
    fn from(value: &RepeatButton) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RepeatButton> for ::windows::core::IInspectable {
    fn from(value: RepeatButton) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RepeatButton> for ::windows::core::IInspectable {
    fn from(value: &RepeatButton) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RepeatButton> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: RepeatButton) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RepeatButton> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &RepeatButton) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RepeatButton> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: RepeatButton) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RepeatButton> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &RepeatButton) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<RepeatButton> for ButtonBase {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::Into::<ButtonBase>::into(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for ButtonBase {
    fn from(value: &RepeatButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ButtonBase> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, ButtonBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<ButtonBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ButtonBase> for &RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, ButtonBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<ButtonBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RepeatButton> for super::ContentControl {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::Into::<super::ContentControl>::into(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::ContentControl {
    fn from(value: &RepeatButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for &RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RepeatButton> for super::Control {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::Control {
    fn from(value: &RepeatButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RepeatButton> for super::super::FrameworkElement {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::super::FrameworkElement {
    fn from(value: &RepeatButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RepeatButton> for super::super::UIElement {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::super::UIElement {
    fn from(value: &RepeatButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RepeatButton> for super::super::DependencyObject {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::super::DependencyObject {
    fn from(value: &RepeatButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &RepeatButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RepeatButton {}
unsafe impl ::core::marker::Sync for RepeatButton {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScrollBar(pub ::windows::core::IInspectable);
impl ScrollBar {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScrollBar, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<super::Orientation> {
        let this = self;
        unsafe {
            let mut result__: super::Orientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Orientation>(result__)
        }
    }
    pub fn SetOrientation(&self, value: super::Orientation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ViewportSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetViewportSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IndicatorMode(&self) -> ::windows::core::Result<ScrollingIndicatorMode> {
        let this = self;
        unsafe {
            let mut result__: ScrollingIndicatorMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ScrollingIndicatorMode>(result__)
        }
    }
    pub fn SetIndicatorMode(&self, value: ScrollingIndicatorMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Scroll<'a, Param0: ::windows::core::IntoParam<'a, ScrollEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveScroll<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn OrientationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IScrollBarStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ViewportSizeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IScrollBarStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IndicatorModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IScrollBarStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IScrollBarStatics<R, F: FnOnce(&IScrollBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScrollBar, IScrollBarStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ScrollBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ScrollBar;{f57ae6ca-d1a6-4b90-a4e9-54df1ba8d2ec})");
}
unsafe impl ::windows::core::Interface for ScrollBar {
    type Vtable = IScrollBar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf57ae6ca_d1a6_4b90_a4e9_54df1ba8d2ec);
}
impl ::windows::core::RuntimeName for ScrollBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ScrollBar";
}
impl ::core::convert::From<ScrollBar> for ::windows::core::IUnknown {
    fn from(value: ScrollBar) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScrollBar> for ::windows::core::IUnknown {
    fn from(value: &ScrollBar) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScrollBar> for ::windows::core::IInspectable {
    fn from(value: ScrollBar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScrollBar> for ::windows::core::IInspectable {
    fn from(value: &ScrollBar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ScrollBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: ScrollBar) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ScrollBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &ScrollBar) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ScrollBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: ScrollBar) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ScrollBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &ScrollBar) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ScrollBar> for RangeBase {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::Into::<RangeBase>::into(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for RangeBase {
    fn from(value: &ScrollBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, RangeBase> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, RangeBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<RangeBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, RangeBase> for &ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, RangeBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<RangeBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ScrollBar> for super::Control {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for super::Control {
    fn from(value: &ScrollBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ScrollBar> for super::super::FrameworkElement {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for super::super::FrameworkElement {
    fn from(value: &ScrollBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ScrollBar> for super::super::UIElement {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for super::super::UIElement {
    fn from(value: &ScrollBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ScrollBar> for super::super::DependencyObject {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for super::super::DependencyObject {
    fn from(value: &ScrollBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ScrollBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ScrollBar {}
unsafe impl ::core::marker::Sync for ScrollBar {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScrollEventArgs(pub ::windows::core::IInspectable);
impl ScrollEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScrollEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NewValue(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ScrollEventType(&self) -> ::windows::core::Result<ScrollEventType> {
        let this = self;
        unsafe {
            let mut result__: ScrollEventType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ScrollEventType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ScrollEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ScrollEventArgs;{c57e5168-3afe-448d-b752-2f364c75d743})");
}
unsafe impl ::windows::core::Interface for ScrollEventArgs {
    type Vtable = IScrollEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc57e5168_3afe_448d_b752_2f364c75d743);
}
impl ::windows::core::RuntimeName for ScrollEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ScrollEventArgs";
}
impl ::core::convert::From<ScrollEventArgs> for ::windows::core::IUnknown {
    fn from(value: ScrollEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScrollEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ScrollEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScrollEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScrollEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScrollEventArgs> for ::windows::core::IInspectable {
    fn from(value: ScrollEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScrollEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ScrollEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScrollEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScrollEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ScrollEventArgs> for super::super::RoutedEventArgs {
    fn from(value: ScrollEventArgs) -> Self {
        ::core::convert::Into::<super::super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ScrollEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &ScrollEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for ScrollEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::RoutedEventArgs> for &ScrollEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ScrollEventArgs {}
unsafe impl ::core::marker::Sync for ScrollEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScrollEventHandler(::windows::core::IUnknown);
impl ScrollEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ScrollEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = ScrollEventHandler_box::<F> {
            vtable: &ScrollEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ScrollEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ScrollEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({8860b0a4-a383-4c83-b306-a1c39d7db87f})");
}
unsafe impl ::windows::core::Interface for ScrollEventHandler {
    type Vtable = ScrollEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8860b0a4_a383_4c83_b306_a1c39d7db87f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ScrollEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct ScrollEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ScrollEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const ScrollEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ScrollEventArgs>) -> ::windows::core::Result<()> + 'static> ScrollEventHandler_box<F> {
    const VTABLE: ScrollEventHandler_abi = ScrollEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ScrollEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <ScrollEventArgs as ::windows::core::Abi>::Abi as *const <ScrollEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ScrollEventType(pub i32);
impl ScrollEventType {
    pub const SmallDecrement: ScrollEventType = ScrollEventType(0i32);
    pub const SmallIncrement: ScrollEventType = ScrollEventType(1i32);
    pub const LargeDecrement: ScrollEventType = ScrollEventType(2i32);
    pub const LargeIncrement: ScrollEventType = ScrollEventType(3i32);
    pub const ThumbPosition: ScrollEventType = ScrollEventType(4i32);
    pub const ThumbTrack: ScrollEventType = ScrollEventType(5i32);
    pub const First: ScrollEventType = ScrollEventType(6i32);
    pub const Last: ScrollEventType = ScrollEventType(7i32);
    pub const EndScroll: ScrollEventType = ScrollEventType(8i32);
}
impl ::core::convert::From<i32> for ScrollEventType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ScrollEventType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ScrollEventType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ScrollEventType;i4)");
}
impl ::windows::core::DefaultType for ScrollEventType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ScrollingIndicatorMode(pub i32);
impl ScrollingIndicatorMode {
    pub const None: ScrollingIndicatorMode = ScrollingIndicatorMode(0i32);
    pub const TouchIndicator: ScrollingIndicatorMode = ScrollingIndicatorMode(1i32);
    pub const MouseIndicator: ScrollingIndicatorMode = ScrollingIndicatorMode(2i32);
}
impl ::core::convert::From<i32> for ScrollingIndicatorMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ScrollingIndicatorMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ScrollingIndicatorMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ScrollingIndicatorMode;i4)");
}
impl ::windows::core::DefaultType for ScrollingIndicatorMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Selector(pub ::windows::core::IInspectable);
impl Selector {
    pub fn SelectedIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetSelectedItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetSelectedValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedValuePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSelectedValuePath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsSynchronizedWithCurrentItem(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetIsSynchronizedWithCurrentItem<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::SelectionChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn SelectedIndexProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedItemProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedValuePathProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsSynchronizedWithCurrentItemProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsSelectionActive<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ISelectorStatics<R, F: FnOnce(&ISelectorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Selector, ISelectorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Selector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.Selector;{e30eb3a5-b36b-42dc-8527-cd25136c083c})");
}
unsafe impl ::windows::core::Interface for Selector {
    type Vtable = ISelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe30eb3a5_b36b_42dc_8527_cd25136c083c);
}
impl ::windows::core::RuntimeName for Selector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Selector";
}
impl ::core::convert::From<Selector> for ::windows::core::IUnknown {
    fn from(value: Selector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Selector> for ::windows::core::IUnknown {
    fn from(value: &Selector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Selector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Selector> for ::windows::core::IInspectable {
    fn from(value: Selector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Selector> for ::windows::core::IInspectable {
    fn from(value: &Selector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Selector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<Selector> for super::IItemContainerMapping {
    type Error = ::windows::core::Error;
    fn try_from(value: Selector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Selector> for super::IItemContainerMapping {
    type Error = ::windows::core::Error;
    fn try_from(value: &Selector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IItemContainerMapping> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::IItemContainerMapping> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IItemContainerMapping> for &Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::IItemContainerMapping> {
        ::core::convert::TryInto::<super::IItemContainerMapping>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Selector> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Selector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Selector> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Selector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Selector> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Selector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Selector> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Selector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Selector> for super::ItemsControl {
    fn from(value: Selector) -> Self {
        ::core::convert::Into::<super::ItemsControl>::into(&value)
    }
}
impl ::core::convert::From<&Selector> for super::ItemsControl {
    fn from(value: &Selector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ItemsControl> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::ItemsControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ItemsControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ItemsControl> for &Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::ItemsControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ItemsControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Selector> for super::Control {
    fn from(value: Selector) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&Selector> for super::Control {
    fn from(value: &Selector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Selector> for super::super::FrameworkElement {
    fn from(value: Selector) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Selector> for super::super::FrameworkElement {
    fn from(value: &Selector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Selector> for super::super::UIElement {
    fn from(value: Selector) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Selector> for super::super::UIElement {
    fn from(value: &Selector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Selector> for super::super::DependencyObject {
    fn from(value: Selector) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Selector> for super::super::DependencyObject {
    fn from(value: &Selector) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Selector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Selector {}
unsafe impl ::core::marker::Sync for Selector {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SelectorItem(pub ::windows::core::IInspectable);
impl SelectorItem {
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsSelectedProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISelectorItemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISelectorItemStatics<R, F: FnOnce(&ISelectorItemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SelectorItem, ISelectorItemStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SelectorItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.SelectorItem;{541c8d6c-0283-4581-b945-2a64c28a0646})");
}
unsafe impl ::windows::core::Interface for SelectorItem {
    type Vtable = ISelectorItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x541c8d6c_0283_4581_b945_2a64c28a0646);
}
impl ::windows::core::RuntimeName for SelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SelectorItem";
}
impl ::core::convert::From<SelectorItem> for ::windows::core::IUnknown {
    fn from(value: SelectorItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SelectorItem> for ::windows::core::IUnknown {
    fn from(value: &SelectorItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SelectorItem> for ::windows::core::IInspectable {
    fn from(value: SelectorItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SelectorItem> for ::windows::core::IInspectable {
    fn from(value: &SelectorItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<SelectorItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SelectorItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&SelectorItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SelectorItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<SelectorItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: SelectorItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&SelectorItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &SelectorItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<SelectorItem> for super::ContentControl {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::Into::<super::ContentControl>::into(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::ContentControl {
    fn from(value: &SelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for &SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SelectorItem> for super::Control {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::Control {
    fn from(value: &SelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SelectorItem> for super::super::FrameworkElement {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::super::FrameworkElement {
    fn from(value: &SelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SelectorItem> for super::super::UIElement {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::super::UIElement {
    fn from(value: &SelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SelectorItem> for super::super::DependencyObject {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::super::DependencyObject {
    fn from(value: &SelectorItem) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SelectorItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SelectorItem {}
unsafe impl ::core::marker::Sync for SelectorItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SettingsFlyoutTemplateSettings(pub ::windows::core::IInspectable);
impl SettingsFlyoutTemplateSettings {
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn HeaderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn HeaderForeground(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn BorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    pub fn BorderThickness(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn IconSource(&self) -> ::windows::core::Result<super::super::Media::ImageSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::ImageSource>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub fn ContentTransitions(&self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Animation::TransitionCollection>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SettingsFlyoutTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.SettingsFlyoutTemplateSettings;{bcf14c10-cea7-43f1-9d68-57605ded69d4})");
}
unsafe impl ::windows::core::Interface for SettingsFlyoutTemplateSettings {
    type Vtable = ISettingsFlyoutTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcf14c10_cea7_43f1_9d68_57605ded69d4);
}
impl ::windows::core::RuntimeName for SettingsFlyoutTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SettingsFlyoutTemplateSettings";
}
impl ::core::convert::From<SettingsFlyoutTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: SettingsFlyoutTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SettingsFlyoutTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &SettingsFlyoutTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SettingsFlyoutTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: SettingsFlyoutTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SettingsFlyoutTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &SettingsFlyoutTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SettingsFlyoutTemplateSettings> for super::super::DependencyObject {
    fn from(value: SettingsFlyoutTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SettingsFlyoutTemplateSettings> for super::super::DependencyObject {
    fn from(value: &SettingsFlyoutTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SettingsFlyoutTemplateSettings {}
unsafe impl ::core::marker::Sync for SettingsFlyoutTemplateSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SliderSnapsTo(pub i32);
impl SliderSnapsTo {
    pub const StepValues: SliderSnapsTo = SliderSnapsTo(0i32);
    pub const Ticks: SliderSnapsTo = SliderSnapsTo(1i32);
}
impl ::core::convert::From<i32> for SliderSnapsTo {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SliderSnapsTo {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SliderSnapsTo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.SliderSnapsTo;i4)");
}
impl ::windows::core::DefaultType for SliderSnapsTo {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SnapPointsAlignment(pub i32);
impl SnapPointsAlignment {
    pub const Near: SnapPointsAlignment = SnapPointsAlignment(0i32);
    pub const Center: SnapPointsAlignment = SnapPointsAlignment(1i32);
    pub const Far: SnapPointsAlignment = SnapPointsAlignment(2i32);
}
impl ::core::convert::From<i32> for SnapPointsAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SnapPointsAlignment {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SnapPointsAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.SnapPointsAlignment;i4)");
}
impl ::windows::core::DefaultType for SnapPointsAlignment {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SplitViewTemplateSettings(pub ::windows::core::IInspectable);
impl SplitViewTemplateSettings {
    pub fn OpenPaneLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeOpenPaneLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OpenPaneLengthMinusCompactLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeOpenPaneLengthMinusCompactLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OpenPaneGridLength(&self) -> ::windows::core::Result<super::super::GridLength> {
        let this = self;
        unsafe {
            let mut result__: super::super::GridLength = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::GridLength>(result__)
        }
    }
    pub fn CompactPaneGridLength(&self) -> ::windows::core::Result<super::super::GridLength> {
        let this = self;
        unsafe {
            let mut result__: super::super::GridLength = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::GridLength>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SplitViewTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.SplitViewTemplateSettings;{c16ab5a7-4996-4443-b199-6b6b89124eab})");
}
unsafe impl ::windows::core::Interface for SplitViewTemplateSettings {
    type Vtable = ISplitViewTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc16ab5a7_4996_4443_b199_6b6b89124eab);
}
impl ::windows::core::RuntimeName for SplitViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SplitViewTemplateSettings";
}
impl ::core::convert::From<SplitViewTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: SplitViewTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SplitViewTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &SplitViewTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplitViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SplitViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SplitViewTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: SplitViewTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SplitViewTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &SplitViewTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplitViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SplitViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SplitViewTemplateSettings> for super::super::DependencyObject {
    fn from(value: SplitViewTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SplitViewTemplateSettings> for super::super::DependencyObject {
    fn from(value: &SplitViewTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplitViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplitViewTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SplitViewTemplateSettings {}
unsafe impl ::core::marker::Sync for SplitViewTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Thumb(pub ::windows::core::IInspectable);
impl Thumb {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Thumb, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsDragging(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DragStarted<'a, Param0: ::windows::core::IntoParam<'a, DragStartedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDragStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DragDelta<'a, Param0: ::windows::core::IntoParam<'a, DragDeltaEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDragDelta<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DragCompleted<'a, Param0: ::windows::core::IntoParam<'a, DragCompletedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDragCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CancelDrag(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn IsDraggingProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IThumbStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IThumbStatics<R, F: FnOnce(&IThumbStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Thumb, IThumbStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Thumb {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.Thumb;{e8b2b281-0d6a-45cf-b333-2402b037f099})");
}
unsafe impl ::windows::core::Interface for Thumb {
    type Vtable = IThumb_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8b2b281_0d6a_45cf_b333_2402b037f099);
}
impl ::windows::core::RuntimeName for Thumb {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Thumb";
}
impl ::core::convert::From<Thumb> for ::windows::core::IUnknown {
    fn from(value: Thumb) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Thumb> for ::windows::core::IUnknown {
    fn from(value: &Thumb) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Thumb> for ::windows::core::IInspectable {
    fn from(value: Thumb) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Thumb> for ::windows::core::IInspectable {
    fn from(value: &Thumb) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Thumb> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Thumb) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Thumb> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Thumb) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Thumb> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Thumb) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Thumb> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Thumb) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Thumb> for super::Control {
    fn from(value: Thumb) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&Thumb> for super::Control {
    fn from(value: &Thumb) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Thumb> for super::super::FrameworkElement {
    fn from(value: Thumb) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Thumb> for super::super::FrameworkElement {
    fn from(value: &Thumb) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Thumb> for super::super::UIElement {
    fn from(value: Thumb) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Thumb> for super::super::UIElement {
    fn from(value: &Thumb) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Thumb> for super::super::DependencyObject {
    fn from(value: Thumb) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Thumb> for super::super::DependencyObject {
    fn from(value: &Thumb) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Thumb {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Thumb {}
unsafe impl ::core::marker::Sync for Thumb {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TickBar(pub ::windows::core::IInspectable);
impl TickBar {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TickBar, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows::core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FillProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITickBarStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ITickBarStatics<R, F: FnOnce(&ITickBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TickBar, ITickBarStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TickBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.TickBar;{994683fa-f1f6-487d-a5ac-c15921bfa995})");
}
unsafe impl ::windows::core::Interface for TickBar {
    type Vtable = ITickBar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x994683fa_f1f6_487d_a5ac_c15921bfa995);
}
impl ::windows::core::RuntimeName for TickBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.TickBar";
}
impl ::core::convert::From<TickBar> for ::windows::core::IUnknown {
    fn from(value: TickBar) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TickBar> for ::windows::core::IUnknown {
    fn from(value: &TickBar) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TickBar> for ::windows::core::IInspectable {
    fn from(value: TickBar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TickBar> for ::windows::core::IInspectable {
    fn from(value: &TickBar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<TickBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: TickBar) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&TickBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &TickBar) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<TickBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: TickBar) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&TickBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &TickBar) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<TickBar> for super::super::FrameworkElement {
    fn from(value: TickBar) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&TickBar> for super::super::FrameworkElement {
    fn from(value: &TickBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<TickBar> for super::super::UIElement {
    fn from(value: TickBar) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&TickBar> for super::super::UIElement {
    fn from(value: &TickBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<TickBar> for super::super::DependencyObject {
    fn from(value: TickBar) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TickBar> for super::super::DependencyObject {
    fn from(value: &TickBar) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &TickBar {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for TickBar {}
unsafe impl ::core::marker::Sync for TickBar {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TickPlacement(pub i32);
impl TickPlacement {
    pub const None: TickPlacement = TickPlacement(0i32);
    pub const TopLeft: TickPlacement = TickPlacement(1i32);
    pub const BottomRight: TickPlacement = TickPlacement(2i32);
    pub const Outside: TickPlacement = TickPlacement(3i32);
    pub const Inline: TickPlacement = TickPlacement(4i32);
}
impl ::core::convert::From<i32> for TickPlacement {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TickPlacement {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TickPlacement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.TickPlacement;i4)");
}
impl ::windows::core::DefaultType for TickPlacement {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToggleButton(pub ::windows::core::IInspectable);
impl ToggleButton {
    #[cfg(feature = "Foundation")]
    pub fn IsChecked(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetIsChecked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsThreeState(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsThreeState(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Checked<'a, Param0: ::windows::core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveChecked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Unchecked<'a, Param0: ::windows::core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnchecked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Indeterminate<'a, Param0: ::windows::core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveIndeterminate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IsCheckedProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IToggleButtonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsThreeStateProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IToggleButtonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<ToggleButton> {
        Self::IToggleButtonFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ToggleButton>(result__)
        })
    }
    pub fn IToggleButtonStatics<R, F: FnOnce(&IToggleButtonStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToggleButton, IToggleButtonStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToggleButtonFactory<R, F: FnOnce(&IToggleButtonFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToggleButton, IToggleButtonFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ToggleButton {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ToggleButton;{589877fb-0fc7-4036-9d8b-127dfa75c16d})");
}
unsafe impl ::windows::core::Interface for ToggleButton {
    type Vtable = IToggleButton_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x589877fb_0fc7_4036_9d8b_127dfa75c16d);
}
impl ::windows::core::RuntimeName for ToggleButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToggleButton";
}
impl ::core::convert::From<ToggleButton> for ::windows::core::IUnknown {
    fn from(value: ToggleButton) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToggleButton> for ::windows::core::IUnknown {
    fn from(value: &ToggleButton) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToggleButton> for ::windows::core::IInspectable {
    fn from(value: ToggleButton) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToggleButton> for ::windows::core::IInspectable {
    fn from(value: &ToggleButton) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ToggleButton> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: ToggleButton) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ToggleButton> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToggleButton) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ToggleButton> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: ToggleButton) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ToggleButton> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToggleButton) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ToggleButton> for ButtonBase {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::Into::<ButtonBase>::into(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for ButtonBase {
    fn from(value: &ToggleButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ButtonBase> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, ButtonBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<ButtonBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ButtonBase> for &ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, ButtonBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<ButtonBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ToggleButton> for super::ContentControl {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::Into::<super::ContentControl>::into(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::ContentControl {
    fn from(value: &ToggleButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ContentControl> for &ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::ContentControl> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ToggleButton> for super::Control {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::Into::<super::Control>::into(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::Control {
    fn from(value: &ToggleButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ToggleButton> for super::super::FrameworkElement {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::Into::<super::super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::super::FrameworkElement {
    fn from(value: &ToggleButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ToggleButton> for super::super::UIElement {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::Into::<super::super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::super::UIElement {
    fn from(value: &ToggleButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ToggleButton> for super::super::DependencyObject {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::super::DependencyObject {
    fn from(value: &ToggleButton) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ToggleButton {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ToggleButton {}
unsafe impl ::core::marker::Sync for ToggleButton {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToggleSwitchTemplateSettings(pub ::windows::core::IInspectable);
impl ToggleSwitchTemplateSettings {
    pub fn KnobCurrentToOnOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn KnobCurrentToOffOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn KnobOnToOffOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn KnobOffToOnOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CurtainCurrentToOnOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CurtainCurrentToOffOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CurtainOnToOffOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CurtainOffToOnOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToggleSwitchTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ToggleSwitchTemplateSettings;{02b7bdcd-628a-4363-86e0-51d6e2e89e58})");
}
unsafe impl ::windows::core::Interface for ToggleSwitchTemplateSettings {
    type Vtable = IToggleSwitchTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b7bdcd_628a_4363_86e0_51d6e2e89e58);
}
impl ::windows::core::RuntimeName for ToggleSwitchTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToggleSwitchTemplateSettings";
}
impl ::core::convert::From<ToggleSwitchTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: ToggleSwitchTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToggleSwitchTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &ToggleSwitchTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToggleSwitchTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: ToggleSwitchTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToggleSwitchTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &ToggleSwitchTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ToggleSwitchTemplateSettings> for super::super::DependencyObject {
    fn from(value: ToggleSwitchTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ToggleSwitchTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ToggleSwitchTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ToggleSwitchTemplateSettings {}
unsafe impl ::core::marker::Sync for ToggleSwitchTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToolTipTemplateSettings(pub ::windows::core::IInspectable);
impl ToolTipTemplateSettings {
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToolTipTemplateSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ToolTipTemplateSettings;{d4388247-0ec4-4506-affd-afac2225b48c})");
}
unsafe impl ::windows::core::Interface for ToolTipTemplateSettings {
    type Vtable = IToolTipTemplateSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4388247_0ec4_4506_affd_afac2225b48c);
}
impl ::windows::core::RuntimeName for ToolTipTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToolTipTemplateSettings";
}
impl ::core::convert::From<ToolTipTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: ToolTipTemplateSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToolTipTemplateSettings> for ::windows::core::IUnknown {
    fn from(value: &ToolTipTemplateSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToolTipTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToolTipTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToolTipTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: ToolTipTemplateSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToolTipTemplateSettings> for ::windows::core::IInspectable {
    fn from(value: &ToolTipTemplateSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToolTipTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToolTipTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ToolTipTemplateSettings> for super::super::DependencyObject {
    fn from(value: ToolTipTemplateSettings) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ToolTipTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ToolTipTemplateSettings) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ToolTipTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ToolTipTemplateSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ToolTipTemplateSettings {}
unsafe impl ::core::marker::Sync for ToolTipTemplateSettings {}
