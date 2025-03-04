#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Xaml_Media_Animation")]
pub mod Animation;
#[cfg(feature = "UI_Xaml_Media_Imaging")]
pub mod Imaging;
#[cfg(feature = "UI_Xaml_Media_Media3D")]
pub mod Media3D;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AcrylicBackgroundSource(pub i32);
impl AcrylicBackgroundSource {
    pub const HostBackdrop: AcrylicBackgroundSource = AcrylicBackgroundSource(0i32);
    pub const Backdrop: AcrylicBackgroundSource = AcrylicBackgroundSource(1i32);
}
impl ::core::convert::From<i32> for AcrylicBackgroundSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AcrylicBackgroundSource {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AcrylicBackgroundSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AcrylicBackgroundSource;i4)");
}
impl ::windows::core::DefaultType for AcrylicBackgroundSource {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AcrylicBrush(pub ::windows::core::IInspectable);
impl AcrylicBrush {
    pub fn BackgroundSource(&self) -> ::windows::core::Result<AcrylicBackgroundSource> {
        let this = self;
        unsafe {
            let mut result__: AcrylicBackgroundSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AcrylicBackgroundSource>(result__)
        }
    }
    pub fn SetBackgroundSource(&self, value: AcrylicBackgroundSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TintColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetTintColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TintOpacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TintTransitionDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTintTransitionDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AlwaysUseFallback(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysUseFallback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BackgroundSourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TintColorProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TintOpacityProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TintTransitionDurationProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AlwaysUseFallbackProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<AcrylicBrush> {
        Self::IAcrylicBrushFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<AcrylicBrush>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn TintLuminosityOpacity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<IAcrylicBrush2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTintLuminosityOpacity<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAcrylicBrush2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TintLuminosityOpacityProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IAcrylicBrushStatics<R, F: FnOnce(&IAcrylicBrushStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AcrylicBrush, IAcrylicBrushStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAcrylicBrushFactory<R, F: FnOnce(&IAcrylicBrushFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AcrylicBrush, IAcrylicBrushFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAcrylicBrushStatics2<R, F: FnOnce(&IAcrylicBrushStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AcrylicBrush, IAcrylicBrushStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AcrylicBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.AcrylicBrush;{79bbcf4e-cd66-4f1b-a8b6-cd6d2977c18d})");
}
unsafe impl ::windows::core::Interface for AcrylicBrush {
    type Vtable = IAcrylicBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79bbcf4e_cd66_4f1b_a8b6_cd6d2977c18d);
}
impl ::windows::core::RuntimeName for AcrylicBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.AcrylicBrush";
}
impl ::core::convert::From<AcrylicBrush> for ::windows::core::IUnknown {
    fn from(value: AcrylicBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AcrylicBrush> for ::windows::core::IUnknown {
    fn from(value: &AcrylicBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AcrylicBrush> for ::windows::core::IInspectable {
    fn from(value: AcrylicBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AcrylicBrush> for ::windows::core::IInspectable {
    fn from(value: &AcrylicBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<AcrylicBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: AcrylicBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&AcrylicBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &AcrylicBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<AcrylicBrush> for XamlCompositionBrushBase {
    fn from(value: AcrylicBrush) -> Self {
        ::core::convert::Into::<XamlCompositionBrushBase>::into(&value)
    }
}
impl ::core::convert::From<&AcrylicBrush> for XamlCompositionBrushBase {
    fn from(value: &AcrylicBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlCompositionBrushBase> for AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, XamlCompositionBrushBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlCompositionBrushBase> for &AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, XamlCompositionBrushBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<AcrylicBrush> for Brush {
    fn from(value: AcrylicBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&AcrylicBrush> for Brush {
    fn from(value: &AcrylicBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<AcrylicBrush> for super::DependencyObject {
    fn from(value: AcrylicBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&AcrylicBrush> for super::DependencyObject {
    fn from(value: &AcrylicBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &AcrylicBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for AcrylicBrush {}
unsafe impl ::core::marker::Sync for AcrylicBrush {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AlignmentX(pub i32);
impl AlignmentX {
    pub const Left: AlignmentX = AlignmentX(0i32);
    pub const Center: AlignmentX = AlignmentX(1i32);
    pub const Right: AlignmentX = AlignmentX(2i32);
}
impl ::core::convert::From<i32> for AlignmentX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AlignmentX {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AlignmentX {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AlignmentX;i4)");
}
impl ::windows::core::DefaultType for AlignmentX {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AlignmentY(pub i32);
impl AlignmentY {
    pub const Top: AlignmentY = AlignmentY(0i32);
    pub const Center: AlignmentY = AlignmentY(1i32);
    pub const Bottom: AlignmentY = AlignmentY(2i32);
}
impl ::core::convert::From<i32> for AlignmentY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AlignmentY {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AlignmentY {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AlignmentY;i4)");
}
impl ::windows::core::DefaultType for AlignmentY {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ArcSegment(pub ::windows::core::IInspectable);
impl ArcSegment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ArcSegment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RotationAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsLargeArc(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLargeArc(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SweepDirection(&self) -> ::windows::core::Result<SweepDirection> {
        let this = self;
        unsafe {
            let mut result__: SweepDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SweepDirection>(result__)
        }
    }
    pub fn SetSweepDirection(&self, value: SweepDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PointProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationAngleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsLargeArcProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SweepDirectionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IArcSegmentStatics<R, F: FnOnce(&IArcSegmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ArcSegment, IArcSegmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ArcSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ArcSegment;{07940c5f-63fb-4469-91be-f1097c168052})");
}
unsafe impl ::windows::core::Interface for ArcSegment {
    type Vtable = IArcSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07940c5f_63fb_4469_91be_f1097c168052);
}
impl ::windows::core::RuntimeName for ArcSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ArcSegment";
}
impl ::core::convert::From<ArcSegment> for ::windows::core::IUnknown {
    fn from(value: ArcSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ArcSegment> for ::windows::core::IUnknown {
    fn from(value: &ArcSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ArcSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ArcSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ArcSegment> for ::windows::core::IInspectable {
    fn from(value: ArcSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ArcSegment> for ::windows::core::IInspectable {
    fn from(value: &ArcSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ArcSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ArcSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ArcSegment> for PathSegment {
    fn from(value: ArcSegment) -> Self {
        ::core::convert::Into::<PathSegment>::into(&value)
    }
}
impl ::core::convert::From<&ArcSegment> for PathSegment {
    fn from(value: &ArcSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for ArcSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for &ArcSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ArcSegment> for super::DependencyObject {
    fn from(value: ArcSegment) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ArcSegment> for super::DependencyObject {
    fn from(value: &ArcSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for ArcSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &ArcSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ArcSegment {}
unsafe impl ::core::marker::Sync for ArcSegment {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioCategory(pub i32);
impl AudioCategory {
    pub const Other: AudioCategory = AudioCategory(0i32);
    pub const ForegroundOnlyMedia: AudioCategory = AudioCategory(1i32);
    pub const BackgroundCapableMedia: AudioCategory = AudioCategory(2i32);
    pub const Communications: AudioCategory = AudioCategory(3i32);
    pub const Alerts: AudioCategory = AudioCategory(4i32);
    pub const SoundEffects: AudioCategory = AudioCategory(5i32);
    pub const GameEffects: AudioCategory = AudioCategory(6i32);
    pub const GameMedia: AudioCategory = AudioCategory(7i32);
    pub const GameChat: AudioCategory = AudioCategory(8i32);
    pub const Speech: AudioCategory = AudioCategory(9i32);
    pub const Movie: AudioCategory = AudioCategory(10i32);
    pub const Media: AudioCategory = AudioCategory(11i32);
}
impl ::core::convert::From<i32> for AudioCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioCategory {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AudioCategory;i4)");
}
impl ::windows::core::DefaultType for AudioCategory {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioDeviceType(pub i32);
impl AudioDeviceType {
    pub const Console: AudioDeviceType = AudioDeviceType(0i32);
    pub const Multimedia: AudioDeviceType = AudioDeviceType(1i32);
    pub const Communications: AudioDeviceType = AudioDeviceType(2i32);
}
impl ::core::convert::From<i32> for AudioDeviceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioDeviceType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AudioDeviceType;i4)");
}
impl ::windows::core::DefaultType for AudioDeviceType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BezierSegment(pub ::windows::core::IInspectable);
impl BezierSegment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BezierSegment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Point1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint1<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Point2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Point3(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint3<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Point1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Point2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Point3Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IBezierSegmentStatics<R, F: FnOnce(&IBezierSegmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BezierSegment, IBezierSegmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BezierSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.BezierSegment;{af4bb9ee-8984-49b7-81df-3f35994b95eb})");
}
unsafe impl ::windows::core::Interface for BezierSegment {
    type Vtable = IBezierSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf4bb9ee_8984_49b7_81df_3f35994b95eb);
}
impl ::windows::core::RuntimeName for BezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BezierSegment";
}
impl ::core::convert::From<BezierSegment> for ::windows::core::IUnknown {
    fn from(value: BezierSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BezierSegment> for ::windows::core::IUnknown {
    fn from(value: &BezierSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BezierSegment> for ::windows::core::IInspectable {
    fn from(value: BezierSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BezierSegment> for ::windows::core::IInspectable {
    fn from(value: &BezierSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BezierSegment> for PathSegment {
    fn from(value: BezierSegment) -> Self {
        ::core::convert::Into::<PathSegment>::into(&value)
    }
}
impl ::core::convert::From<&BezierSegment> for PathSegment {
    fn from(value: &BezierSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for BezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for &BezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<BezierSegment> for super::DependencyObject {
    fn from(value: BezierSegment) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BezierSegment> for super::DependencyObject {
    fn from(value: &BezierSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for BezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &BezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for BezierSegment {}
unsafe impl ::core::marker::Sync for BezierSegment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapCache(pub ::windows::core::IInspectable);
impl BitmapCache {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapCache, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapCache {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.BitmapCache;{79c2219e-44d2-4610-9735-9bec83809ecf})");
}
unsafe impl ::windows::core::Interface for BitmapCache {
    type Vtable = IBitmapCache_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c2219e_44d2_4610_9735_9bec83809ecf);
}
impl ::windows::core::RuntimeName for BitmapCache {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BitmapCache";
}
impl ::core::convert::From<BitmapCache> for ::windows::core::IUnknown {
    fn from(value: BitmapCache) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapCache> for ::windows::core::IUnknown {
    fn from(value: &BitmapCache) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapCache {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapCache {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapCache> for ::windows::core::IInspectable {
    fn from(value: BitmapCache) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapCache> for ::windows::core::IInspectable {
    fn from(value: &BitmapCache) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapCache {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapCache {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BitmapCache> for CacheMode {
    fn from(value: BitmapCache) -> Self {
        ::core::convert::Into::<CacheMode>::into(&value)
    }
}
impl ::core::convert::From<&BitmapCache> for CacheMode {
    fn from(value: &BitmapCache) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, CacheMode> for BitmapCache {
    fn into_param(self) -> ::windows::core::Param<'a, CacheMode> {
        ::windows::core::Param::Owned(::core::convert::Into::<CacheMode>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, CacheMode> for &BitmapCache {
    fn into_param(self) -> ::windows::core::Param<'a, CacheMode> {
        ::windows::core::Param::Owned(::core::convert::Into::<CacheMode>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<BitmapCache> for super::DependencyObject {
    fn from(value: BitmapCache) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BitmapCache> for super::DependencyObject {
    fn from(value: &BitmapCache) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for BitmapCache {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &BitmapCache {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for BitmapCache {}
unsafe impl ::core::marker::Sync for BitmapCache {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Brush(pub ::windows::core::IInspectable);
impl Brush {
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Transform(&self) -> ::windows::core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Transform>(result__)
        }
    }
    pub fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, Transform>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTransform(&self) -> ::windows::core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Transform>(result__)
        }
    }
    pub fn SetRelativeTransform<'a, Param0: ::windows::core::IntoParam<'a, Transform>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn OpacityProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TransformProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RelativeTransformProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Composition::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn IBrushStatics<R, F: FnOnce(&IBrushStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Brush, IBrushStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Brush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Brush;{8806a321-1e06-422c-a1cc-01696559e021})");
}
unsafe impl ::windows::core::Interface for Brush {
    type Vtable = IBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8806a321_1e06_422c_a1cc_01696559e021);
}
impl ::windows::core::RuntimeName for Brush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Brush";
}
impl ::core::convert::From<Brush> for ::windows::core::IUnknown {
    fn from(value: Brush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Brush> for ::windows::core::IUnknown {
    fn from(value: &Brush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Brush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Brush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Brush> for ::windows::core::IInspectable {
    fn from(value: Brush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Brush> for ::windows::core::IInspectable {
    fn from(value: &Brush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Brush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Brush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Brush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Brush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Brush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Brush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Brush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Brush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Brush> for super::DependencyObject {
    fn from(value: Brush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Brush> for super::DependencyObject {
    fn from(value: &Brush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Brush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Brush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Brush {}
unsafe impl ::core::marker::Sync for Brush {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BrushCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl BrushCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BrushCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Brush>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Brush>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Brush>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Brush>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Brush>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Brush>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Brush as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Brush as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<Brush>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<Brush>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<Brush>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for BrushCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.BrushCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Brush;{8806a321-1e06-422c-a1cc-01696559e021})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for BrushCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<Brush>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<Brush> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for BrushCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BrushCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<BrushCollection> for ::windows::core::IUnknown {
    fn from(value: BrushCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&BrushCollection> for ::windows::core::IUnknown {
    fn from(value: &BrushCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BrushCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BrushCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<BrushCollection> for ::windows::core::IInspectable {
    fn from(value: BrushCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&BrushCollection> for ::windows::core::IInspectable {
    fn from(value: &BrushCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BrushCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BrushCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<BrushCollection> for super::super::super::Foundation::Collections::IVector<Brush> {
    fn from(value: BrushCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&BrushCollection> for super::super::super::Foundation::Collections::IVector<Brush> {
    fn from(value: &BrushCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Brush>> for BrushCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Brush>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Brush>> for &BrushCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Brush>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<BrushCollection> for super::super::super::Foundation::Collections::IIterable<Brush> {
    type Error = ::windows::core::Error;
    fn try_from(value: BrushCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&BrushCollection> for super::super::super::Foundation::Collections::IIterable<Brush> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BrushCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Brush>> for BrushCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Brush>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Brush>> for &BrushCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Brush>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<Brush>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for BrushCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for BrushCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for BrushCollection {
    type Item = Brush;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &BrushCollection {
    type Item = Brush;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BrushMappingMode(pub i32);
impl BrushMappingMode {
    pub const Absolute: BrushMappingMode = BrushMappingMode(0i32);
    pub const RelativeToBoundingBox: BrushMappingMode = BrushMappingMode(1i32);
}
impl ::core::convert::From<i32> for BrushMappingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BrushMappingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BrushMappingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.BrushMappingMode;i4)");
}
impl ::windows::core::DefaultType for BrushMappingMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CacheMode(pub ::windows::core::IInspectable);
impl CacheMode {}
unsafe impl ::windows::core::RuntimeType for CacheMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.CacheMode;{98dc8b11-c6f9-4dab-b838-5fd5ec8c7350})");
}
unsafe impl ::windows::core::Interface for CacheMode {
    type Vtable = ICacheMode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98dc8b11_c6f9_4dab_b838_5fd5ec8c7350);
}
impl ::windows::core::RuntimeName for CacheMode {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CacheMode";
}
impl ::core::convert::From<CacheMode> for ::windows::core::IUnknown {
    fn from(value: CacheMode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CacheMode> for ::windows::core::IUnknown {
    fn from(value: &CacheMode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CacheMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CacheMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CacheMode> for ::windows::core::IInspectable {
    fn from(value: CacheMode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CacheMode> for ::windows::core::IInspectable {
    fn from(value: &CacheMode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CacheMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CacheMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CacheMode> for super::DependencyObject {
    fn from(value: CacheMode) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CacheMode> for super::DependencyObject {
    fn from(value: &CacheMode) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for CacheMode {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &CacheMode {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CacheMode {}
unsafe impl ::core::marker::Sync for CacheMode {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ColorInterpolationMode(pub i32);
impl ColorInterpolationMode {
    pub const ScRgbLinearInterpolation: ColorInterpolationMode = ColorInterpolationMode(0i32);
    pub const SRgbLinearInterpolation: ColorInterpolationMode = ColorInterpolationMode(1i32);
}
impl ::core::convert::From<i32> for ColorInterpolationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ColorInterpolationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ColorInterpolationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.ColorInterpolationMode;i4)");
}
impl ::windows::core::DefaultType for ColorInterpolationMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompositeTransform(pub ::windows::core::IInspectable);
impl CompositeTransform {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompositeTransform, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SkewX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetSkewX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SkewY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetSkewY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotation(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TranslateX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TranslateY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SkewXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SkewYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ICompositeTransformStatics<R, F: FnOnce(&ICompositeTransformStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompositeTransform, ICompositeTransformStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CompositeTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.CompositeTransform;{c8a4385b-f24a-4701-a265-a78846f142b9})");
}
unsafe impl ::windows::core::Interface for CompositeTransform {
    type Vtable = ICompositeTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a4385b_f24a_4701_a265_a78846f142b9);
}
impl ::windows::core::RuntimeName for CompositeTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CompositeTransform";
}
impl ::core::convert::From<CompositeTransform> for ::windows::core::IUnknown {
    fn from(value: CompositeTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositeTransform> for ::windows::core::IUnknown {
    fn from(value: &CompositeTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositeTransform> for ::windows::core::IInspectable {
    fn from(value: CompositeTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositeTransform> for ::windows::core::IInspectable {
    fn from(value: &CompositeTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CompositeTransform> for Transform {
    fn from(value: CompositeTransform) -> Self {
        ::core::convert::Into::<Transform>::into(&value)
    }
}
impl ::core::convert::From<&CompositeTransform> for Transform {
    fn from(value: &CompositeTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for &CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CompositeTransform> for GeneralTransform {
    fn from(value: CompositeTransform) -> Self {
        ::core::convert::Into::<GeneralTransform>::into(&value)
    }
}
impl ::core::convert::From<&CompositeTransform> for GeneralTransform {
    fn from(value: &CompositeTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for &CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CompositeTransform> for super::DependencyObject {
    fn from(value: CompositeTransform) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CompositeTransform> for super::DependencyObject {
    fn from(value: &CompositeTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &CompositeTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CompositeTransform {}
unsafe impl ::core::marker::Sync for CompositeTransform {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompositionTarget(pub ::windows::core::IInspectable);
impl CompositionTarget {
    #[cfg(feature = "Foundation")]
    pub fn Rendering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRendering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICompositionTargetStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn SurfaceContentsLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSurfaceContentsLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICompositionTargetStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn Rendered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<RenderedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::ICompositionTargetStatics3(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRendered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICompositionTargetStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn ICompositionTargetStatics<R, F: FnOnce(&ICompositionTargetStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompositionTarget, ICompositionTargetStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICompositionTargetStatics3<R, F: FnOnce(&ICompositionTargetStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompositionTarget, ICompositionTargetStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.CompositionTarget;{26cfbff0-713c-4bec-8803-e101f7b14ed3})");
}
unsafe impl ::windows::core::Interface for CompositionTarget {
    type Vtable = ICompositionTarget_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26cfbff0_713c_4bec_8803_e101f7b14ed3);
}
impl ::windows::core::RuntimeName for CompositionTarget {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CompositionTarget";
}
impl ::core::convert::From<CompositionTarget> for ::windows::core::IUnknown {
    fn from(value: CompositionTarget) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositionTarget> for ::windows::core::IUnknown {
    fn from(value: &CompositionTarget) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositionTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompositionTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositionTarget> for ::windows::core::IInspectable {
    fn from(value: CompositionTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositionTarget> for ::windows::core::IInspectable {
    fn from(value: &CompositionTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositionTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompositionTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CompositionTarget {}
unsafe impl ::core::marker::Sync for CompositionTarget {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl DoubleCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: f64, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<f64>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<f64>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<f64>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for DoubleCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.DoubleCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};f8))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for DoubleCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<f64>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<f64> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for DoubleCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.DoubleCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DoubleCollection> for ::windows::core::IUnknown {
    fn from(value: DoubleCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DoubleCollection> for ::windows::core::IUnknown {
    fn from(value: &DoubleCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DoubleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DoubleCollection> for ::windows::core::IInspectable {
    fn from(value: DoubleCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DoubleCollection> for ::windows::core::IInspectable {
    fn from(value: &DoubleCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DoubleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DoubleCollection> for super::super::super::Foundation::Collections::IVector<f64> {
    fn from(value: DoubleCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DoubleCollection> for super::super::super::Foundation::Collections::IVector<f64> {
    fn from(value: &DoubleCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<f64>> for DoubleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<f64>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<f64>> for &DoubleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<f64>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DoubleCollection> for super::super::super::Foundation::Collections::IIterable<f64> {
    type Error = ::windows::core::Error;
    fn try_from(value: DoubleCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DoubleCollection> for super::super::super::Foundation::Collections::IIterable<f64> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DoubleCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<f64>> for DoubleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<f64>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<f64>> for &DoubleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<f64>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<f64>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for DoubleCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DoubleCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for DoubleCollection {
    type Item = f64;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &DoubleCollection {
    type Item = f64;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ElementCompositeMode(pub i32);
impl ElementCompositeMode {
    pub const Inherit: ElementCompositeMode = ElementCompositeMode(0i32);
    pub const SourceOver: ElementCompositeMode = ElementCompositeMode(1i32);
    pub const MinBlend: ElementCompositeMode = ElementCompositeMode(2i32);
}
impl ::core::convert::From<i32> for ElementCompositeMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ElementCompositeMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ElementCompositeMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.ElementCompositeMode;i4)");
}
impl ::windows::core::DefaultType for ElementCompositeMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EllipseGeometry(pub ::windows::core::IInspectable);
impl EllipseGeometry {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EllipseGeometry, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetCenter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RadiusX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadiusX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RadiusY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadiusY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RadiusXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RadiusYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IEllipseGeometryStatics<R, F: FnOnce(&IEllipseGeometryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EllipseGeometry, IEllipseGeometryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EllipseGeometry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.EllipseGeometry;{d4f61bba-4ea2-40d6-aa6c-8d38aa87651f})");
}
unsafe impl ::windows::core::Interface for EllipseGeometry {
    type Vtable = IEllipseGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4f61bba_4ea2_40d6_aa6c_8d38aa87651f);
}
impl ::windows::core::RuntimeName for EllipseGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.EllipseGeometry";
}
impl ::core::convert::From<EllipseGeometry> for ::windows::core::IUnknown {
    fn from(value: EllipseGeometry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EllipseGeometry> for ::windows::core::IUnknown {
    fn from(value: &EllipseGeometry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EllipseGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EllipseGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EllipseGeometry> for ::windows::core::IInspectable {
    fn from(value: EllipseGeometry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EllipseGeometry> for ::windows::core::IInspectable {
    fn from(value: &EllipseGeometry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EllipseGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EllipseGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<EllipseGeometry> for Geometry {
    fn from(value: EllipseGeometry) -> Self {
        ::core::convert::Into::<Geometry>::into(&value)
    }
}
impl ::core::convert::From<&EllipseGeometry> for Geometry {
    fn from(value: &EllipseGeometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for EllipseGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for &EllipseGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<EllipseGeometry> for super::DependencyObject {
    fn from(value: EllipseGeometry) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&EllipseGeometry> for super::DependencyObject {
    fn from(value: &EllipseGeometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for EllipseGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &EllipseGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for EllipseGeometry {}
unsafe impl ::core::marker::Sync for EllipseGeometry {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FastPlayFallbackBehaviour(pub i32);
impl FastPlayFallbackBehaviour {
    pub const Skip: FastPlayFallbackBehaviour = FastPlayFallbackBehaviour(0i32);
    pub const Hide: FastPlayFallbackBehaviour = FastPlayFallbackBehaviour(1i32);
    pub const Disable: FastPlayFallbackBehaviour = FastPlayFallbackBehaviour(2i32);
}
impl ::core::convert::From<i32> for FastPlayFallbackBehaviour {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FastPlayFallbackBehaviour {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FastPlayFallbackBehaviour {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.FastPlayFallbackBehaviour;i4)");
}
impl ::windows::core::DefaultType for FastPlayFallbackBehaviour {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FillRule(pub i32);
impl FillRule {
    pub const EvenOdd: FillRule = FillRule(0i32);
    pub const Nonzero: FillRule = FillRule(1i32);
}
impl ::core::convert::From<i32> for FillRule {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FillRule {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FillRule {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.FillRule;i4)");
}
impl ::windows::core::DefaultType for FillRule {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FontFamily(pub ::windows::core::IInspectable);
impl FontFamily {
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateInstanceWithName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(familyname: Param0) -> ::windows::core::Result<FontFamily> {
        Self::IFontFamilyFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), familyname.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<FontFamily>(result__)
        })
    }
    pub fn XamlAutoFontFamily() -> ::windows::core::Result<FontFamily> {
        Self::IFontFamilyStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontFamily>(result__)
        })
    }
    pub fn IFontFamilyFactory<R, F: FnOnce(&IFontFamilyFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FontFamily, IFontFamilyFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFontFamilyStatics2<R, F: FnOnce(&IFontFamilyStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FontFamily, IFontFamilyStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FontFamily {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.FontFamily;{92467e64-d66a-4cf4-9322-3d23b3c0c361})");
}
unsafe impl ::windows::core::Interface for FontFamily {
    type Vtable = IFontFamily_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92467e64_d66a_4cf4_9322_3d23b3c0c361);
}
impl ::windows::core::RuntimeName for FontFamily {
    const NAME: &'static str = "Windows.UI.Xaml.Media.FontFamily";
}
impl ::core::convert::From<FontFamily> for ::windows::core::IUnknown {
    fn from(value: FontFamily) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FontFamily> for ::windows::core::IUnknown {
    fn from(value: &FontFamily) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FontFamily {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FontFamily {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FontFamily> for ::windows::core::IInspectable {
    fn from(value: FontFamily) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FontFamily> for ::windows::core::IInspectable {
    fn from(value: &FontFamily) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FontFamily {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FontFamily {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FontFamily {}
unsafe impl ::core::marker::Sync for FontFamily {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeneralTransform(pub ::windows::core::IInspectable);
impl GeneralTransform {
    pub fn Inverse(&self) -> ::windows::core::Result<GeneralTransform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeneralTransform>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TransformPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, point: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), point.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, inpoint: Param0, outpoint: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), inpoint.into_param().abi(), outpoint, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TransformBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, rect: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), rect.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GeneralTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GeneralTransform;{a06798b7-a2ec-415f-ade2-eade9333f2c7})");
}
unsafe impl ::windows::core::Interface for GeneralTransform {
    type Vtable = IGeneralTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06798b7_a2ec_415f_ade2_eade9333f2c7);
}
impl ::windows::core::RuntimeName for GeneralTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeneralTransform";
}
impl ::core::convert::From<GeneralTransform> for ::windows::core::IUnknown {
    fn from(value: GeneralTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeneralTransform> for ::windows::core::IUnknown {
    fn from(value: &GeneralTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeneralTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeneralTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeneralTransform> for ::windows::core::IInspectable {
    fn from(value: GeneralTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeneralTransform> for ::windows::core::IInspectable {
    fn from(value: &GeneralTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeneralTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeneralTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<GeneralTransform> for super::DependencyObject {
    fn from(value: GeneralTransform) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&GeneralTransform> for super::DependencyObject {
    fn from(value: &GeneralTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for GeneralTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &GeneralTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GeneralTransform {}
unsafe impl ::core::marker::Sync for GeneralTransform {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geometry(pub ::windows::core::IInspectable);
impl Geometry {
    pub fn Transform(&self) -> ::windows::core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Transform>(result__)
        }
    }
    pub fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, Transform>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn Empty() -> ::windows::core::Result<Geometry> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Geometry>(result__)
        })
    }
    pub fn StandardFlatteningTolerance() -> ::windows::core::Result<f64> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    pub fn TransformProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IGeometryStatics<R, F: FnOnce(&IGeometryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geometry, IGeometryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Geometry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Geometry;{fa123889-0acd-417b-b62d-5ca1bf4dfc0e})");
}
unsafe impl ::windows::core::Interface for Geometry {
    type Vtable = IGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa123889_0acd_417b_b62d_5ca1bf4dfc0e);
}
impl ::windows::core::RuntimeName for Geometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Geometry";
}
impl ::core::convert::From<Geometry> for ::windows::core::IUnknown {
    fn from(value: Geometry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geometry> for ::windows::core::IUnknown {
    fn from(value: &Geometry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geometry> for ::windows::core::IInspectable {
    fn from(value: Geometry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geometry> for ::windows::core::IInspectable {
    fn from(value: &Geometry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Geometry> for super::DependencyObject {
    fn from(value: Geometry) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Geometry> for super::DependencyObject {
    fn from(value: &Geometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Geometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Geometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Geometry {}
unsafe impl ::core::marker::Sync for Geometry {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeometryCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl GeometryCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeometryCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Geometry> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Geometry>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Geometry>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Geometry>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Geometry>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Geometry>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Geometry>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Geometry>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Geometry as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Geometry as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<Geometry>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<Geometry>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<Geometry>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for GeometryCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GeometryCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Geometry;{fa123889-0acd-417b-b62d-5ca1bf4dfc0e})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for GeometryCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<Geometry>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<Geometry> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for GeometryCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeometryCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<GeometryCollection> for ::windows::core::IUnknown {
    fn from(value: GeometryCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&GeometryCollection> for ::windows::core::IUnknown {
    fn from(value: &GeometryCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeometryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeometryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<GeometryCollection> for ::windows::core::IInspectable {
    fn from(value: GeometryCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&GeometryCollection> for ::windows::core::IInspectable {
    fn from(value: &GeometryCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeometryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeometryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<GeometryCollection> for super::super::super::Foundation::Collections::IVector<Geometry> {
    fn from(value: GeometryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&GeometryCollection> for super::super::super::Foundation::Collections::IVector<Geometry> {
    fn from(value: &GeometryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Geometry>> for GeometryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Geometry>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Geometry>> for &GeometryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Geometry>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<GeometryCollection> for super::super::super::Foundation::Collections::IIterable<Geometry> {
    type Error = ::windows::core::Error;
    fn try_from(value: GeometryCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&GeometryCollection> for super::super::super::Foundation::Collections::IIterable<Geometry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GeometryCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Geometry>> for GeometryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Geometry>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Geometry>> for &GeometryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Geometry>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<Geometry>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for GeometryCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for GeometryCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for GeometryCollection {
    type Item = Geometry;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &GeometryCollection {
    type Item = Geometry;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeometryGroup(pub ::windows::core::IInspectable);
impl GeometryGroup {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeometryGroup, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FillRule(&self) -> ::windows::core::Result<FillRule> {
        let this = self;
        unsafe {
            let mut result__: FillRule = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FillRule>(result__)
        }
    }
    pub fn SetFillRule(&self, value: FillRule) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<GeometryCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeometryCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetChildren<'a, Param0: ::windows::core::IntoParam<'a, GeometryCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FillRuleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGeometryGroupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ChildrenProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGeometryGroupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IGeometryGroupStatics<R, F: FnOnce(&IGeometryGroupStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeometryGroup, IGeometryGroupStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GeometryGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GeometryGroup;{55225a61-8677-4c8c-8e46-ee3dc355114b})");
}
unsafe impl ::windows::core::Interface for GeometryGroup {
    type Vtable = IGeometryGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55225a61_8677_4c8c_8e46_ee3dc355114b);
}
impl ::windows::core::RuntimeName for GeometryGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeometryGroup";
}
impl ::core::convert::From<GeometryGroup> for ::windows::core::IUnknown {
    fn from(value: GeometryGroup) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeometryGroup> for ::windows::core::IUnknown {
    fn from(value: &GeometryGroup) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeometryGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeometryGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeometryGroup> for ::windows::core::IInspectable {
    fn from(value: GeometryGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeometryGroup> for ::windows::core::IInspectable {
    fn from(value: &GeometryGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeometryGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeometryGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<GeometryGroup> for Geometry {
    fn from(value: GeometryGroup) -> Self {
        ::core::convert::Into::<Geometry>::into(&value)
    }
}
impl ::core::convert::From<&GeometryGroup> for Geometry {
    fn from(value: &GeometryGroup) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for GeometryGroup {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for &GeometryGroup {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<GeometryGroup> for super::DependencyObject {
    fn from(value: GeometryGroup) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&GeometryGroup> for super::DependencyObject {
    fn from(value: &GeometryGroup) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for GeometryGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &GeometryGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GeometryGroup {}
unsafe impl ::core::marker::Sync for GeometryGroup {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GradientBrush(pub ::windows::core::IInspectable);
impl GradientBrush {
    pub fn SpreadMethod(&self) -> ::windows::core::Result<GradientSpreadMethod> {
        let this = self;
        unsafe {
            let mut result__: GradientSpreadMethod = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GradientSpreadMethod>(result__)
        }
    }
    pub fn SetSpreadMethod(&self, value: GradientSpreadMethod) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MappingMode(&self) -> ::windows::core::Result<BrushMappingMode> {
        let this = self;
        unsafe {
            let mut result__: BrushMappingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BrushMappingMode>(result__)
        }
    }
    pub fn SetMappingMode(&self, value: BrushMappingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ColorInterpolationMode(&self) -> ::windows::core::Result<ColorInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__: ColorInterpolationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ColorInterpolationMode>(result__)
        }
    }
    pub fn SetColorInterpolationMode(&self, value: ColorInterpolationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GradientStops(&self) -> ::windows::core::Result<GradientStopCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GradientStopCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetGradientStops<'a, Param0: ::windows::core::IntoParam<'a, GradientStopCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SpreadMethodProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn MappingModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ColorInterpolationModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GradientStopsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IGradientBrushStatics<R, F: FnOnce(&IGradientBrushStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GradientBrush, IGradientBrushStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GradientBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GradientBrush;{2166e69f-935a-4191-8e3c-1c8dfdfcdc78})");
}
unsafe impl ::windows::core::Interface for GradientBrush {
    type Vtable = IGradientBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2166e69f_935a_4191_8e3c_1c8dfdfcdc78);
}
impl ::windows::core::RuntimeName for GradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientBrush";
}
impl ::core::convert::From<GradientBrush> for ::windows::core::IUnknown {
    fn from(value: GradientBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GradientBrush> for ::windows::core::IUnknown {
    fn from(value: &GradientBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GradientBrush> for ::windows::core::IInspectable {
    fn from(value: GradientBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GradientBrush> for ::windows::core::IInspectable {
    fn from(value: &GradientBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<GradientBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: GradientBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&GradientBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &GradientBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<GradientBrush> for Brush {
    fn from(value: GradientBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&GradientBrush> for Brush {
    fn from(value: &GradientBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<GradientBrush> for super::DependencyObject {
    fn from(value: GradientBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&GradientBrush> for super::DependencyObject {
    fn from(value: &GradientBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &GradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GradientBrush {}
unsafe impl ::core::marker::Sync for GradientBrush {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GradientSpreadMethod(pub i32);
impl GradientSpreadMethod {
    pub const Pad: GradientSpreadMethod = GradientSpreadMethod(0i32);
    pub const Reflect: GradientSpreadMethod = GradientSpreadMethod(1i32);
    pub const Repeat: GradientSpreadMethod = GradientSpreadMethod(2i32);
}
impl ::core::convert::From<i32> for GradientSpreadMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GradientSpreadMethod {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GradientSpreadMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.GradientSpreadMethod;i4)");
}
impl ::windows::core::DefaultType for GradientSpreadMethod {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GradientStop(pub ::windows::core::IInspectable);
impl GradientStop {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GradientStop, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Color(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Offset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ColorProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGradientStopStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGradientStopStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IGradientStopStatics<R, F: FnOnce(&IGradientStopStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GradientStop, IGradientStopStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GradientStop {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GradientStop;{665f44fe-2e59-4c4a-ab53-076a100ccd81})");
}
unsafe impl ::windows::core::Interface for GradientStop {
    type Vtable = IGradientStop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x665f44fe_2e59_4c4a_ab53_076a100ccd81);
}
impl ::windows::core::RuntimeName for GradientStop {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientStop";
}
impl ::core::convert::From<GradientStop> for ::windows::core::IUnknown {
    fn from(value: GradientStop) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GradientStop> for ::windows::core::IUnknown {
    fn from(value: &GradientStop) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GradientStop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GradientStop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GradientStop> for ::windows::core::IInspectable {
    fn from(value: GradientStop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GradientStop> for ::windows::core::IInspectable {
    fn from(value: &GradientStop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GradientStop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GradientStop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<GradientStop> for super::DependencyObject {
    fn from(value: GradientStop) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&GradientStop> for super::DependencyObject {
    fn from(value: &GradientStop) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for GradientStop {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &GradientStop {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GradientStop {}
unsafe impl ::core::marker::Sync for GradientStop {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GradientStopCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl GradientStopCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GradientStopCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<GradientStop> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<GradientStop>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GradientStop>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GradientStop>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, GradientStop>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, GradientStop>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, GradientStop>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, GradientStop>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<GradientStop as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<GradientStop as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<GradientStop>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<GradientStop>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<GradientStop>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for GradientStopCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GradientStopCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.GradientStop;{665f44fe-2e59-4c4a-ab53-076a100ccd81})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for GradientStopCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<GradientStop>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<GradientStop> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for GradientStopCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientStopCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<GradientStopCollection> for ::windows::core::IUnknown {
    fn from(value: GradientStopCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&GradientStopCollection> for ::windows::core::IUnknown {
    fn from(value: &GradientStopCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<GradientStopCollection> for ::windows::core::IInspectable {
    fn from(value: GradientStopCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&GradientStopCollection> for ::windows::core::IInspectable {
    fn from(value: &GradientStopCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<GradientStopCollection> for super::super::super::Foundation::Collections::IVector<GradientStop> {
    fn from(value: GradientStopCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&GradientStopCollection> for super::super::super::Foundation::Collections::IVector<GradientStop> {
    fn from(value: &GradientStopCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<GradientStop>> for GradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<GradientStop>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<GradientStop>> for &GradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<GradientStop>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<GradientStopCollection> for super::super::super::Foundation::Collections::IIterable<GradientStop> {
    type Error = ::windows::core::Error;
    fn try_from(value: GradientStopCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&GradientStopCollection> for super::super::super::Foundation::Collections::IIterable<GradientStop> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GradientStopCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<GradientStop>> for GradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<GradientStop>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<GradientStop>> for &GradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<GradientStop>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<GradientStop>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for GradientStopCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for GradientStopCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for GradientStopCollection {
    type Item = GradientStop;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &GradientStopCollection {
    type Item = GradientStop;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAcrylicBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAcrylicBrush {
    type Vtable = IAcrylicBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79bbcf4e_cd66_4f1b_a8b6_cd6d2977c18d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AcrylicBackgroundSource) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AcrylicBackgroundSource) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAcrylicBrush2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAcrylicBrush2 {
    type Vtable = IAcrylicBrush2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9645383_b19e_5ac0_86ff_3d90506dbcda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrush2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAcrylicBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAcrylicBrushFactory {
    type Vtable = IAcrylicBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81a32568_f6cc_4013_8363_928ae23b7a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushFactory_abi(
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
pub struct IAcrylicBrushStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAcrylicBrushStatics {
    type Vtable = IAcrylicBrushStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2787fd79_a3da_423f_b81a_599147971523);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushStatics_abi(
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
pub struct IAcrylicBrushStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAcrylicBrushStatics2 {
    type Vtable = IAcrylicBrushStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x129188a8_bf11_5bbc_8445_8c510e5926c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushStatics2_abi(
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
pub struct IArcSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IArcSegment {
    type Vtable = IArcSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07940c5f_63fb_4469_91be_f1097c168052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SweepDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: SweepDirection) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IArcSegmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IArcSegmentStatics {
    type Vtable = IArcSegmentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82348f6e_8a69_4204_9c12_7207df317643);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegmentStatics_abi(
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
pub struct IBezierSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBezierSegment {
    type Vtable = IBezierSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf4bb9ee_8984_49b7_81df_3f35994b95eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBezierSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBezierSegmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBezierSegmentStatics {
    type Vtable = IBezierSegmentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0287bac_1410_4530_8452_1c9d0ad1f341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBezierSegmentStatics_abi(
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
pub struct IBitmapCache(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapCache {
    type Vtable = IBitmapCache_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c2219e_44d2_4610_9735_9bec83809ecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCache_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBrush {
    type Vtable = IBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8806a321_1e06_422c_a1cc_01696559e021);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBrushFactory {
    type Vtable = IBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x399658a2_14fb_4b8f_83e6_6e3dab12069b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushFactory_abi(
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
pub struct IBrushOverrides2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBrushOverrides2 {
    type Vtable = IBrushOverrides2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd092b151_d83b_5a81_a71e_a1c7f8ad6963);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushOverrides2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animationpropertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBrushStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBrushStatics {
    type Vtable = IBrushStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe70c3102_0225_47f5_b22e_0467619f6a22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushStatics_abi(
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
pub struct ICacheMode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICacheMode {
    type Vtable = ICacheMode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98dc8b11_c6f9_4dab_b838_5fd5ec8c7350);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICacheMode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICacheModeFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICacheModeFactory {
    type Vtable = ICacheModeFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb1f8c5b_0abb_4e70_b8a8_620d0d953ab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICacheModeFactory_abi(
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
pub struct ICompositeTransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositeTransform {
    type Vtable = ICompositeTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a4385b_f24a_4701_a265_a78846f142b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositeTransformStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositeTransformStatics {
    type Vtable = ICompositeTransformStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f190c08_8266_496f_9653_a18bd4f836aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransformStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionTarget(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionTarget {
    type Vtable = ICompositionTarget_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26cfbff0_713c_4bec_8803_e101f7b14ed3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionTargetStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionTargetStatics {
    type Vtable = ICompositionTargetStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b1af03d_1ed2_4b59_bd00_7594ee92832b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionTargetStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionTargetStatics3 {
    type Vtable = ICompositionTargetStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc0a7cd9_6750_4708_994c_2028e0312ac8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEllipseGeometry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEllipseGeometry {
    type Vtable = IEllipseGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4f61bba_4ea2_40d6_aa6c_8d38aa87651f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipseGeometry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEllipseGeometryStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEllipseGeometryStatics {
    type Vtable = IEllipseGeometryStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1744db47_f635_4b16_aee6_e052a65defb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipseGeometryStatics_abi(
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
pub struct IFontFamily(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFontFamily {
    type Vtable = IFontFamily_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92467e64_d66a_4cf4_9322_3d23b3c0c361);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamily_abi(
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
pub struct IFontFamilyFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFontFamilyFactory {
    type Vtable = IFontFamilyFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5603377_3dae_4dcd_af09_f9498e9ec659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, familyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFontFamilyStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFontFamilyStatics2 {
    type Vtable = IFontFamilyStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52ad7af9_37e6_4297_a238_97fb6a408d9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyStatics2_abi(
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
pub struct IGeneralTransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeneralTransform {
    type Vtable = IGeneralTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06798b7_a2ec_415f_ade2_eade9333f2c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, point: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeneralTransformFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeneralTransformFactory {
    type Vtable = IGeneralTransformFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a25c930_29c4_4e31_b6f9_dedd52e4df1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransformFactory_abi(
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
pub struct IGeneralTransformOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeneralTransformOverrides {
    type Vtable = IGeneralTransformOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f121083_24cf_4524_90ad_8a42b1c12783);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransformOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeometry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeometry {
    type Vtable = IGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa123889_0acd_417b_b62d_5ca1bf4dfc0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeometryFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeometryFactory {
    type Vtable = IGeometryFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf65daf23_d5fd_42f9_b32a_929c5a4b54e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeometryGroup(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeometryGroup {
    type Vtable = IGeometryGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55225a61_8677_4c8c_8e46_ee3dc355114b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FillRule) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FillRule) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeometryGroupStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeometryGroupStatics {
    type Vtable = IGeometryGroupStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56c955f4_8496_4bb6_abf0_617b1fe78b45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryGroupStatics_abi(
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
pub struct IGeometryStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeometryStatics {
    type Vtable = IGeometryStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a70aa8c_0b06_465f_b637_9a47e5a70111);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGradientBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGradientBrush {
    type Vtable = IGradientBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2166e69f_935a_4191_8e3c_1c8dfdfcdc78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GradientSpreadMethod) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GradientSpreadMethod) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BrushMappingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: BrushMappingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ColorInterpolationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ColorInterpolationMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGradientBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGradientBrushFactory {
    type Vtable = IGradientBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed4779ca_45bd_4131_b625_be86e07c6112);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrushFactory_abi(
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
pub struct IGradientBrushStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGradientBrushStatics {
    type Vtable = IGradientBrushStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x961661f9_8bb4_4e6c_b923_b5d787e0f1a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrushStatics_abi(
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
pub struct IGradientStop(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGradientStop {
    type Vtable = IGradientStop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x665f44fe_2e59_4c4a_ab53_076a100ccd81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientStop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGradientStopStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGradientStopStatics {
    type Vtable = IGradientStopStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x602a6d75_6193_4fe5_8e82_c7c6f6febafd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientStopStatics_abi(
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
pub struct IImageBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IImageBrush {
    type Vtable = IImageBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fd11377_c12a_4493_bf7d_f3a8ad74b554);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageBrushStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IImageBrushStatics {
    type Vtable = IImageBrushStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1255b1b2_dd18_42e5_892c_eae30c305b8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBrushStatics_abi(
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
pub struct IImageSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IImageSource {
    type Vtable = IImageSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x737ef309_ea41_4d96_a71c_98e98efcab07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageSourceFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IImageSourceFactory {
    type Vtable = IImageSourceFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297ec001_2540_4e5a_ab66_88035dd3ddb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSourceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineGeometry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineGeometry {
    type Vtable = ILineGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30edd4a2_8fc5_40af_a7a2_c27fe7aa1363);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineGeometry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineGeometryStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineGeometryStatics {
    type Vtable = ILineGeometryStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x578ae763_5562_4ee4_8703_ea4036d891e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineGeometryStatics_abi(
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
pub struct ILineSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineSegment {
    type Vtable = ILineSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6a2e25_3ff0_4420_a411_7182a4cecb15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineSegmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineSegmentStatics {
    type Vtable = ILineSegmentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fcab141_04c0_4afb_87b3_e800b969b894);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegmentStatics_abi(
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
pub struct ILinearGradientBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILinearGradientBrush {
    type Vtable = ILinearGradientBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e96d16b_bb84_4c6f_9dbf_9d6c5c6d9c39);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILinearGradientBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILinearGradientBrushFactory {
    type Vtable = ILinearGradientBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ae0861c_1e7a_4fed_9857_ea8caa798490);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrushFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gradientstopcollection: ::windows::core::RawPtr, angle: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILinearGradientBrushStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILinearGradientBrushStatics {
    type Vtable = ILinearGradientBrushStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7af6e504_2dc3_40e3_be0b_b314c13cb991);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrushStatics_abi(
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
pub struct ILoadedImageSourceLoadCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoadedImageSourceLoadCompletedEventArgs {
    type Vtable = ILoadedImageSourceLoadCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ac60b1e_7837_4489_b3e5_d0d5ad0a56c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSourceLoadCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LoadedImageSourceLoadStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoadedImageSurface(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoadedImageSurface {
    type Vtable = ILoadedImageSurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x050c8313_6737_45ba_8531_33094febef55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSurface_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoadedImageSurfaceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoadedImageSurfaceStatics {
    type Vtable = ILoadedImageSurfaceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22b8edf6_84ad_40ab_937d_4871613e765d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSurfaceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, desiredmaxsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, desiredmaxsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMatrix3DProjection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMatrix3DProjection {
    type Vtable = IMatrix3DProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f03e149_bfc9_4c01_b578_50338cec97fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DProjection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Media3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Media3D::Matrix3D) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))] usize,
    #[cfg(feature = "UI_Xaml_Media_Media3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: Media3D::Matrix3D) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMatrix3DProjectionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMatrix3DProjectionStatics {
    type Vtable = IMatrix3DProjectionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae9d5895_41ec_4e37_abaa_69f41d2f876b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DProjectionStatics_abi(
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
pub struct IMatrixHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMatrixHelper {
    type Vtable = IMatrixHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3cf4882_06b5_48c8_9eb2_1763e9364038);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMatrixHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMatrixHelperStatics {
    type Vtable = IMatrixHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc18606a6_39f4_4b8a_8403_28e5e5f033b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Matrix) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64, result__: *mut Matrix) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: Matrix, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: Matrix, point: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMatrixTransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMatrixTransform {
    type Vtable = IMatrixTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedfdd551_5fed_45fc_ae62_92a4b6cf9707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Matrix) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: Matrix) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMatrixTransformStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMatrixTransformStatics {
    type Vtable = IMatrixTransformStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43e02e47_15b8_4758_bb97_7d52420acc5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixTransformStatics_abi(
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
pub struct IMediaTransportControlsThumbnailRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaTransportControlsThumbnailRequestedEventArgs {
    type Vtable = IMediaTransportControlsThumbnailRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4a8b21c_e3c2_485c_ae69_f1537b76755a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPartialMediaFailureDetectedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPartialMediaFailureDetectedEventArgs {
    type Vtable = IPartialMediaFailureDetectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b65a91_e5a1_442b_88d3_2dc127bfc59b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartialMediaFailureDetectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Media::Playback::FailedMediaStreamKind) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPartialMediaFailureDetectedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPartialMediaFailureDetectedEventArgs2 {
    type Vtable = IPartialMediaFailureDetectedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73074875_890d_416b_b9ae_e84dfd9c4b1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartialMediaFailureDetectedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPathFigure(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathFigure {
    type Vtable = IPathFigure_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d955c8c_5fa9_4dda_a3cc_10fcdcaa20d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigure_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPathFigureStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathFigureStatics {
    type Vtable = IPathFigureStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb60591d9_2395_4317_9552_3a58526f8c7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigureStatics_abi(
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
pub struct IPathGeometry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathGeometry {
    type Vtable = IPathGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x081b9df8_bae6_4bcb_813c_bde0e46dc8b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FillRule) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FillRule) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPathGeometryStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathGeometryStatics {
    type Vtable = IPathGeometryStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9e58bba_2cba_4741_8f8d_3198cf5186b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometryStatics_abi(
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
pub struct IPathSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathSegment {
    type Vtable = IPathSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcfa71cf_9ce3_474f_8157_10b6435a616b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPathSegmentFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathSegmentFactory {
    type Vtable = IPathSegmentFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a1c0aae_eccd_4464_a148_6ffdb3aa281f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegmentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaneProjection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlaneProjection {
    type Vtable = IPlaneProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6f82bfa_6726_469a_b259_a5188347ca8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaneProjection_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Media3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Media3D::Matrix3D) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaneProjectionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlaneProjectionStatics {
    type Vtable = IPlaneProjectionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad919c67_3bdc_4855_8969_d1f9a3adc27d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaneProjectionStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPolyBezierSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolyBezierSegment {
    type Vtable = IPolyBezierSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36805271_38c4_4bcf_96cd_028a6d38af25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyBezierSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPolyBezierSegmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolyBezierSegmentStatics {
    type Vtable = IPolyBezierSegmentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d91a6da_1492_4acc_bd66_a496f3d829d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyBezierSegmentStatics_abi(
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
pub struct IPolyLineSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolyLineSegment {
    type Vtable = IPolyLineSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b397f87_a2e6_479d_bdc8_6f4464646887);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyLineSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPolyLineSegmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolyLineSegmentStatics {
    type Vtable = IPolyLineSegmentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd64a2c87_33f1_4e70_a47f_b4981ef648a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyLineSegmentStatics_abi(
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
pub struct IPolyQuadraticBezierSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolyQuadraticBezierSegment {
    type Vtable = IPolyQuadraticBezierSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd5ced7d_e6db_4c96_b6a1_3fce96e987a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolyQuadraticBezierSegmentStatics {
    type Vtable = IPolyQuadraticBezierSegmentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdf5eb75_7ad5_4c89_8169_8c9786abd9eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegmentStatics_abi(
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
pub struct IProjection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProjection {
    type Vtable = IProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3443557_7f39_4d04_a89c_844338cac897);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProjectionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProjectionFactory {
    type Vtable = IProjectionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4f29cab_60ad_4f24_bd27_9d69c3127c9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjectionFactory_abi(
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
pub struct IQuadraticBezierSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IQuadraticBezierSegment {
    type Vtable = IQuadraticBezierSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c509a5b_bf18_455a_a078_914b5232d8af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticBezierSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IQuadraticBezierSegmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IQuadraticBezierSegmentStatics {
    type Vtable = IQuadraticBezierSegmentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c78278_3c0b_4b4f_b7a2_f003ded41bb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticBezierSegmentStatics_abi(
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
pub struct IRateChangedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRateChangedRoutedEventArgs {
    type Vtable = IRateChangedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9016aa6f_3ca8_4c80_8e2f_8851a68f131f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRateChangedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRectangleGeometry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRectangleGeometry {
    type Vtable = IRectangleGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa25a1f58_c575_4196_91cf_9fdfb10445c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRectangleGeometryStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRectangleGeometryStatics {
    type Vtable = IRectangleGeometryStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x377f8dba_7902_48e3_83be_7c8002a6653c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometryStatics_abi(
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
pub struct IRenderedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRenderedEventArgs {
    type Vtable = IRenderedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe349817d_81c7_4938_828c_a7e2797b35a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRenderingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRenderingEventArgs {
    type Vtable = IRenderingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bf7d30d_9748_4aed_8380_d7890eb776a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRevealBackgroundBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRevealBackgroundBrush {
    type Vtable = IRevealBackgroundBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x261dcc0e_1991_4cdf_aee0_6350a3f90bb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBackgroundBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRevealBackgroundBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRevealBackgroundBrushFactory {
    type Vtable = IRevealBackgroundBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c56bcaa_02a5_4f45_8506_8d39228f5d3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBackgroundBrushFactory_abi(
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
pub struct IRevealBorderBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRevealBorderBrush {
    type Vtable = IRevealBorderBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x060ba115_c542_483c_8202_5f03331866c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBorderBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRevealBorderBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRevealBorderBrushFactory {
    type Vtable = IRevealBorderBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94c25298_f5f8_4482_a25c_6758501a8626);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBorderBrushFactory_abi(
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
pub struct IRevealBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRevealBrush {
    type Vtable = IRevealBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2036a0ed_8271_4398_9019_25872093f13f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::ApplicationTheme) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::ApplicationTheme) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRevealBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRevealBrushFactory {
    type Vtable = IRevealBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d9379ce_e3a0_4aaf_be37_ea9d9dd43105);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrushFactory_abi(
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
pub struct IRevealBrushStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRevealBrushStatics {
    type Vtable = IRevealBrushStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x190f2625_7209_4d42_a847_1ac4bbbb3499);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrushStatics_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: RevealBrushState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut RevealBrushState) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRotateTransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRotateTransform {
    type Vtable = IRotateTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x688ea9b9_1e4e_4596_86e3_428b27334faf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRotateTransform_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRotateTransformStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRotateTransformStatics {
    type Vtable = IRotateTransformStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa131eb8a_51a3_41b6_b9d3_a10e429054ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRotateTransformStatics_abi(
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
pub struct IScaleTransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScaleTransform {
    type Vtable = IScaleTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed67f18d_936e_43ab_929a_e9cd0a511e52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleTransform_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScaleTransformStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScaleTransformStatics {
    type Vtable = IScaleTransformStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d9436f4_40a7_46dd_975a_07d337cd852e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleTransformStatics_abi(
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
pub struct IShadow(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShadow {
    type Vtable = IShadow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6813a583_f3b4_5fcf_8694_2cd0aefc2fad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShadow_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShadowFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShadowFactory {
    type Vtable = IShadowFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19899f25_d28b_51e6_94b0_d7e709686305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShadowFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISkewTransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISkewTransform {
    type Vtable = ISkewTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e8a3b15_7a0f_4617_9e98_1e65bdc92115);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISkewTransform_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISkewTransformStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISkewTransformStatics {
    type Vtable = ISkewTransformStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecd11d73_5614_4b31_b6af_beae10105624);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISkewTransformStatics_abi(
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
pub struct ISolidColorBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISolidColorBrush {
    type Vtable = ISolidColorBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d850850_66f3_48df_9a8f_824bd5e070af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISolidColorBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISolidColorBrushFactory {
    type Vtable = ISolidColorBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd935ce0c_86f5_4da6_8a27_b1619ef7f92b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrushFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: super::super::Color, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISolidColorBrushStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISolidColorBrushStatics {
    type Vtable = ISolidColorBrushStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1a65efa_2b23_41ba_b9ba_7094ec8e4e9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrushStatics_abi(
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
pub struct IThemeShadow(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IThemeShadow {
    type Vtable = IThemeShadow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3eccad09_7985_5f39_8b62_6c10696dca6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeShadow_abi(
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
pub struct IThemeShadowFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IThemeShadowFactory {
    type Vtable = IThemeShadowFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e71465d_0f67_590e_831b_7e5e2a32b778);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeShadowFactory_abi(
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
pub struct ITileBrush(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileBrush {
    type Vtable = ITileBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc201cf06_cd84_48a5_9607_664d7361cd61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AlignmentX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AlignmentX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AlignmentY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AlignmentY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Stretch) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: Stretch) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileBrushFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileBrushFactory {
    type Vtable = ITileBrushFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa159f7c_ed6a_4fb3_b014_b5c7e379a4de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrushFactory_abi(
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
pub struct ITileBrushStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileBrushStatics {
    type Vtable = ITileBrushStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3497c25b_b562_4e68_8435_2399f6eb94d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrushStatics_abi(
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
pub struct ITimelineMarker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimelineMarker {
    type Vtable = ITimelineMarker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa68ef02d_45ba_4e50_8cad_aaea3a227af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITimelineMarkerRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimelineMarkerRoutedEventArgs {
    type Vtable = ITimelineMarkerRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c3b3ef3_2c88_4d9c_99b6_46cdbd48d4c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarkerRoutedEventArgs_abi(
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
pub struct ITimelineMarkerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimelineMarkerStatics {
    type Vtable = ITimelineMarkerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4aef0c6_16a3_484b_87f5_6528b8f04a47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarkerStatics_abi(
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
pub struct ITransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransform {
    type Vtable = ITransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4df74078_bfd6_4ed1_9682_d2fd8bf2fe6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransformFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformFactory {
    type Vtable = ITransformFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a955a66_7cf4_4320_b416_6181192fcc6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransformGroup(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformGroup {
    type Vtable = ITransformGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63418ccc_8d2d_4737_b951_2afce1ddc4c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Matrix) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransformGroupStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformGroupStatics {
    type Vtable = ITransformGroupStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25312f2a_cfab_4b24_9713_5bdead1929c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformGroupStatics_abi(
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
pub struct ITranslateTransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITranslateTransform {
    type Vtable = ITranslateTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc975905c_3c36_4229_817b_178f64c0e113);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITranslateTransform_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITranslateTransformStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITranslateTransformStatics {
    type Vtable = ITranslateTransformStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf419aa91_e042_4111_9c2f_d201304123dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITranslateTransformStatics_abi(
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
pub struct IVisualTreeHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualTreeHelper {
    type Vtable = IVisualTreeHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24b935e3_52c7_4141_8bac_a73d06130569);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualTreeHelperStatics {
    type Vtable = IVisualTreeHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75758c4_d25d_4b1d_971f_596f17f12baa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, intersectingpoint: super::super::super::Foundation::Point, subtree: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, intersectingrect: super::super::super::Foundation::Rect, subtree: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, intersectingpoint: super::super::super::Foundation::Point, subtree: ::windows::core::RawPtr, includeallelements: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, intersectingrect: super::super::super::Foundation::Rect, subtree: ::windows::core::RawPtr, includeallelements: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reference: ::windows::core::RawPtr, childindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reference: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualTreeHelperStatics2 {
    type Vtable = IVisualTreeHelperStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07bcd176_869f_44a7_8797_2103a4c3e47a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualTreeHelperStatics3 {
    type Vtable = IVisualTreeHelperStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40420d50_ca16_57da_8aac_944c8af577fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xamlroot: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlCompositionBrushBase {
    type Vtable = IXamlCompositionBrushBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e432d9_b35c_4a79_811c_c5652004da0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlCompositionBrushBaseFactory {
    type Vtable = IXamlCompositionBrushBaseFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x394f0823_2451_4ed8_bd24_488149b3428d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseFactory_abi(
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
pub struct IXamlCompositionBrushBaseOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlCompositionBrushBaseOverrides {
    type Vtable = IXamlCompositionBrushBaseOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd19127f1_38b4_4ea1_8f33_849629a4c9c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseOverrides_abi(
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
pub struct IXamlCompositionBrushBaseProtected(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlCompositionBrushBaseProtected {
    type Vtable = IXamlCompositionBrushBaseProtected_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1513f3d8_0457_4e1c_ad77_11c1d9879743);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseProtected_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlCompositionBrushBaseStatics {
    type Vtable = IXamlCompositionBrushBaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fd49b06_061a_441f_b97a_adfbd41ae681);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseStatics_abi(
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
pub struct IXamlLight(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlLight {
    type Vtable = IXamlLight_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cc3fc1f_b327_4a18_9648_7c84db26ce22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLight_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlLightFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlLightFactory {
    type Vtable = IXamlLightFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87ded768_3055_43b8_8ef6_798dc4c2329a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightFactory_abi(
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
pub struct IXamlLightOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlLightOverrides {
    type Vtable = IXamlLightOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c6296c7_0173_48e1_b73d_7fa216a9ac28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlLightProtected(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlLightProtected {
    type Vtable = IXamlLightProtected_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ecf220b_1252_43d0_9729_6ea692046838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightProtected_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlLightStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlLightStatics {
    type Vtable = IXamlLightStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ea9d69_b508_4e9c_bd27_6b044b5f78a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ImageBrush(pub ::windows::core::IInspectable);
impl ImageBrush {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ImageBrush, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ImageSource(&self) -> ::windows::core::Result<ImageSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageSource>(result__)
        }
    }
    pub fn SetImageSource<'a, Param0: ::windows::core::IntoParam<'a, ImageSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ImageFailed<'a, Param0: ::windows::core::IntoParam<'a, super::ExceptionRoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveImageFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ImageOpened<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveImageOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ImageSourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IImageBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IImageBrushStatics<R, F: FnOnce(&IImageBrushStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ImageBrush, IImageBrushStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ImageBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ImageBrush;{9fd11377-c12a-4493-bf7d-f3a8ad74b554})");
}
unsafe impl ::windows::core::Interface for ImageBrush {
    type Vtable = IImageBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fd11377_c12a_4493_bf7d_f3a8ad74b554);
}
impl ::windows::core::RuntimeName for ImageBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ImageBrush";
}
impl ::core::convert::From<ImageBrush> for ::windows::core::IUnknown {
    fn from(value: ImageBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ImageBrush> for ::windows::core::IUnknown {
    fn from(value: &ImageBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ImageBrush> for ::windows::core::IInspectable {
    fn from(value: ImageBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ImageBrush> for ::windows::core::IInspectable {
    fn from(value: &ImageBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ImageBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ImageBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ImageBrush> for TileBrush {
    fn from(value: ImageBrush) -> Self {
        ::core::convert::Into::<TileBrush>::into(&value)
    }
}
impl ::core::convert::From<&ImageBrush> for TileBrush {
    fn from(value: &ImageBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TileBrush> for ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, TileBrush> {
        ::windows::core::Param::Owned(::core::convert::Into::<TileBrush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TileBrush> for &ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, TileBrush> {
        ::windows::core::Param::Owned(::core::convert::Into::<TileBrush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ImageBrush> for Brush {
    fn from(value: ImageBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&ImageBrush> for Brush {
    fn from(value: &ImageBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ImageBrush> for super::DependencyObject {
    fn from(value: ImageBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ImageBrush> for super::DependencyObject {
    fn from(value: &ImageBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &ImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ImageBrush {}
unsafe impl ::core::marker::Sync for ImageBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ImageSource(pub ::windows::core::IInspectable);
impl ImageSource {}
unsafe impl ::windows::core::RuntimeType for ImageSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ImageSource;{737ef309-ea41-4d96-a71c-98e98efcab07})");
}
unsafe impl ::windows::core::Interface for ImageSource {
    type Vtable = IImageSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x737ef309_ea41_4d96_a71c_98e98efcab07);
}
impl ::windows::core::RuntimeName for ImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ImageSource";
}
impl ::core::convert::From<ImageSource> for ::windows::core::IUnknown {
    fn from(value: ImageSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ImageSource> for ::windows::core::IUnknown {
    fn from(value: &ImageSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ImageSource> for ::windows::core::IInspectable {
    fn from(value: ImageSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ImageSource> for ::windows::core::IInspectable {
    fn from(value: &ImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ImageSource> for super::DependencyObject {
    fn from(value: ImageSource) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ImageSource> for super::DependencyObject {
    fn from(value: &ImageSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for ImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &ImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ImageSource {}
unsafe impl ::core::marker::Sync for ImageSource {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineGeometry(pub ::windows::core::IInspectable);
impl LineGeometry {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineGeometry, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEndPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StartPointProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn EndPointProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ILineGeometryStatics<R, F: FnOnce(&ILineGeometryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineGeometry, ILineGeometryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LineGeometry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LineGeometry;{30edd4a2-8fc5-40af-a7a2-c27fe7aa1363})");
}
unsafe impl ::windows::core::Interface for LineGeometry {
    type Vtable = ILineGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30edd4a2_8fc5_40af_a7a2_c27fe7aa1363);
}
impl ::windows::core::RuntimeName for LineGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LineGeometry";
}
impl ::core::convert::From<LineGeometry> for ::windows::core::IUnknown {
    fn from(value: LineGeometry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineGeometry> for ::windows::core::IUnknown {
    fn from(value: &LineGeometry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineGeometry> for ::windows::core::IInspectable {
    fn from(value: LineGeometry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineGeometry> for ::windows::core::IInspectable {
    fn from(value: &LineGeometry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LineGeometry> for Geometry {
    fn from(value: LineGeometry) -> Self {
        ::core::convert::Into::<Geometry>::into(&value)
    }
}
impl ::core::convert::From<&LineGeometry> for Geometry {
    fn from(value: &LineGeometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for LineGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for &LineGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LineGeometry> for super::DependencyObject {
    fn from(value: LineGeometry) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LineGeometry> for super::DependencyObject {
    fn from(value: &LineGeometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for LineGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &LineGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LineGeometry {}
unsafe impl ::core::marker::Sync for LineGeometry {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineSegment(pub ::windows::core::IInspectable);
impl LineSegment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineSegment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PointProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ILineSegmentStatics<R, F: FnOnce(&ILineSegmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineSegment, ILineSegmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LineSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LineSegment;{ef6a2e25-3ff0-4420-a411-7182a4cecb15})");
}
unsafe impl ::windows::core::Interface for LineSegment {
    type Vtable = ILineSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6a2e25_3ff0_4420_a411_7182a4cecb15);
}
impl ::windows::core::RuntimeName for LineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LineSegment";
}
impl ::core::convert::From<LineSegment> for ::windows::core::IUnknown {
    fn from(value: LineSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineSegment> for ::windows::core::IUnknown {
    fn from(value: &LineSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineSegment> for ::windows::core::IInspectable {
    fn from(value: LineSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineSegment> for ::windows::core::IInspectable {
    fn from(value: &LineSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LineSegment> for PathSegment {
    fn from(value: LineSegment) -> Self {
        ::core::convert::Into::<PathSegment>::into(&value)
    }
}
impl ::core::convert::From<&LineSegment> for PathSegment {
    fn from(value: &LineSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for LineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for &LineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LineSegment> for super::DependencyObject {
    fn from(value: LineSegment) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LineSegment> for super::DependencyObject {
    fn from(value: &LineSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for LineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &LineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LineSegment {}
unsafe impl ::core::marker::Sync for LineSegment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LinearGradientBrush(pub ::windows::core::IInspectable);
impl LinearGradientBrush {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearGradientBrush, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEndPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstanceWithGradientStopCollectionAndAngle<'a, Param0: ::windows::core::IntoParam<'a, GradientStopCollection>>(gradientstopcollection: Param0, angle: f64) -> ::windows::core::Result<LinearGradientBrush> {
        Self::ILinearGradientBrushFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), gradientstopcollection.into_param().abi(), angle, &mut result__).from_abi::<LinearGradientBrush>(result__)
        })
    }
    pub fn StartPointProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILinearGradientBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn EndPointProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILinearGradientBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ILinearGradientBrushFactory<R, F: FnOnce(&ILinearGradientBrushFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearGradientBrush, ILinearGradientBrushFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILinearGradientBrushStatics<R, F: FnOnce(&ILinearGradientBrushStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearGradientBrush, ILinearGradientBrushStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LinearGradientBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LinearGradientBrush;{8e96d16b-bb84-4c6f-9dbf-9d6c5c6d9c39})");
}
unsafe impl ::windows::core::Interface for LinearGradientBrush {
    type Vtable = ILinearGradientBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e96d16b_bb84_4c6f_9dbf_9d6c5c6d9c39);
}
impl ::windows::core::RuntimeName for LinearGradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LinearGradientBrush";
}
impl ::core::convert::From<LinearGradientBrush> for ::windows::core::IUnknown {
    fn from(value: LinearGradientBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LinearGradientBrush> for ::windows::core::IUnknown {
    fn from(value: &LinearGradientBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LinearGradientBrush> for ::windows::core::IInspectable {
    fn from(value: LinearGradientBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LinearGradientBrush> for ::windows::core::IInspectable {
    fn from(value: &LinearGradientBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LinearGradientBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: LinearGradientBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LinearGradientBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &LinearGradientBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<LinearGradientBrush> for GradientBrush {
    fn from(value: LinearGradientBrush) -> Self {
        ::core::convert::Into::<GradientBrush>::into(&value)
    }
}
impl ::core::convert::From<&LinearGradientBrush> for GradientBrush {
    fn from(value: &LinearGradientBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GradientBrush> for LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, GradientBrush> {
        ::windows::core::Param::Owned(::core::convert::Into::<GradientBrush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GradientBrush> for &LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, GradientBrush> {
        ::windows::core::Param::Owned(::core::convert::Into::<GradientBrush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LinearGradientBrush> for Brush {
    fn from(value: LinearGradientBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&LinearGradientBrush> for Brush {
    fn from(value: &LinearGradientBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LinearGradientBrush> for super::DependencyObject {
    fn from(value: LinearGradientBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LinearGradientBrush> for super::DependencyObject {
    fn from(value: &LinearGradientBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &LinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LinearGradientBrush {}
unsafe impl ::core::marker::Sync for LinearGradientBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoadedImageSourceLoadCompletedEventArgs(pub ::windows::core::IInspectable);
impl LoadedImageSourceLoadCompletedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<LoadedImageSourceLoadStatus> {
        let this = self;
        unsafe {
            let mut result__: LoadedImageSourceLoadStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoadedImageSourceLoadStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LoadedImageSourceLoadCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LoadedImageSourceLoadCompletedEventArgs;{1ac60b1e-7837-4489-b3e5-d0d5ad0a56c4})");
}
unsafe impl ::windows::core::Interface for LoadedImageSourceLoadCompletedEventArgs {
    type Vtable = ILoadedImageSourceLoadCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ac60b1e_7837_4489_b3e5_d0d5ad0a56c4);
}
impl ::windows::core::RuntimeName for LoadedImageSourceLoadCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LoadedImageSourceLoadCompletedEventArgs";
}
impl ::core::convert::From<LoadedImageSourceLoadCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LoadedImageSourceLoadCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoadedImageSourceLoadCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LoadedImageSourceLoadCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoadedImageSourceLoadCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoadedImageSourceLoadCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoadedImageSourceLoadCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LoadedImageSourceLoadCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoadedImageSourceLoadCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LoadedImageSourceLoadCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoadedImageSourceLoadCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoadedImageSourceLoadCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LoadedImageSourceLoadCompletedEventArgs {}
unsafe impl ::core::marker::Sync for LoadedImageSourceLoadCompletedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LoadedImageSourceLoadStatus(pub i32);
impl LoadedImageSourceLoadStatus {
    pub const Success: LoadedImageSourceLoadStatus = LoadedImageSourceLoadStatus(0i32);
    pub const NetworkError: LoadedImageSourceLoadStatus = LoadedImageSourceLoadStatus(1i32);
    pub const InvalidFormat: LoadedImageSourceLoadStatus = LoadedImageSourceLoadStatus(2i32);
    pub const Other: LoadedImageSourceLoadStatus = LoadedImageSourceLoadStatus(3i32);
}
impl ::core::convert::From<i32> for LoadedImageSourceLoadStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LoadedImageSourceLoadStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LoadedImageSourceLoadStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.LoadedImageSourceLoadStatus;i4)");
}
impl ::windows::core::DefaultType for LoadedImageSourceLoadStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoadedImageSurface(pub ::windows::core::IInspectable);
impl LoadedImageSurface {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DecodedPhysicalSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DecodedSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NaturalSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoadCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLoadCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartLoadFromUriWithSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(uri: Param0, desiredmaxsize: Param1) -> ::windows::core::Result<LoadedImageSurface> {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), desiredmaxsize.into_param().abi(), &mut result__).from_abi::<LoadedImageSurface>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn StartLoadFromUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<LoadedImageSurface> {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<LoadedImageSurface>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn StartLoadFromStreamWithSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(stream: Param0, desiredmaxsize: Param1) -> ::windows::core::Result<LoadedImageSurface> {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stream.into_param().abi(), desiredmaxsize.into_param().abi(), &mut result__).from_abi::<LoadedImageSurface>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StartLoadFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IRandomAccessStream>>(stream: Param0) -> ::windows::core::Result<LoadedImageSurface> {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<LoadedImageSurface>(result__)
        })
    }
    pub fn ILoadedImageSurfaceStatics<R, F: FnOnce(&ILoadedImageSurfaceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoadedImageSurface, ILoadedImageSurfaceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LoadedImageSurface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LoadedImageSurface;{050c8313-6737-45ba-8531-33094febef55})");
}
unsafe impl ::windows::core::Interface for LoadedImageSurface {
    type Vtable = ILoadedImageSurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x050c8313_6737_45ba_8531_33094febef55);
}
impl ::windows::core::RuntimeName for LoadedImageSurface {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LoadedImageSurface";
}
impl ::core::convert::From<LoadedImageSurface> for ::windows::core::IUnknown {
    fn from(value: LoadedImageSurface) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoadedImageSurface> for ::windows::core::IUnknown {
    fn from(value: &LoadedImageSurface) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoadedImageSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoadedImageSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoadedImageSurface> for ::windows::core::IInspectable {
    fn from(value: LoadedImageSurface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoadedImageSurface> for ::windows::core::IInspectable {
    fn from(value: &LoadedImageSurface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoadedImageSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoadedImageSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<LoadedImageSurface> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LoadedImageSurface) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LoadedImageSurface> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoadedImageSurface) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for LoadedImageSurface {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &LoadedImageSurface {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoadedImageSurface> for super::super::Composition::ICompositionSurface {
    type Error = ::windows::core::Error;
    fn try_from(value: LoadedImageSurface) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoadedImageSurface> for super::super::Composition::ICompositionSurface {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoadedImageSurface) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::ICompositionSurface> for LoadedImageSurface {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::ICompositionSurface> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::ICompositionSurface> for &LoadedImageSurface {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::ICompositionSurface> {
        ::core::convert::TryInto::<super::super::Composition::ICompositionSurface>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LoadedImageSurface {}
unsafe impl ::core::marker::Sync for LoadedImageSurface {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct Matrix {
    pub M11: f64,
    pub M12: f64,
    pub M21: f64,
    pub M22: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
}
impl Matrix {}
impl ::core::default::Default for Matrix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Matrix {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Matrix").field("M11", &self.M11).field("M12", &self.M12).field("M21", &self.M21).field("M22", &self.M22).field("OffsetX", &self.OffsetX).field("OffsetY", &self.OffsetY).finish()
    }
}
impl ::core::cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M21 == other.M21 && self.M22 == other.M22 && self.OffsetX == other.OffsetX && self.OffsetY == other.OffsetY
    }
}
impl ::core::cmp::Eq for Matrix {}
unsafe impl ::windows::core::Abi for Matrix {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Matrix {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Matrix;f8;f8;f8;f8;f8;f8)");
}
impl ::windows::core::DefaultType for Matrix {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Matrix3DProjection(pub ::windows::core::IInspectable);
impl Matrix3DProjection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Matrix3DProjection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn ProjectionMatrix(&self) -> ::windows::core::Result<Media3D::Matrix3D> {
        let this = self;
        unsafe {
            let mut result__: Media3D::Matrix3D = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Media3D::Matrix3D>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn SetProjectionMatrix<'a, Param0: ::windows::core::IntoParam<'a, Media3D::Matrix3D>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ProjectionMatrixProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IMatrix3DProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IMatrix3DProjectionStatics<R, F: FnOnce(&IMatrix3DProjectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Matrix3DProjection, IMatrix3DProjectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Matrix3DProjection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Matrix3DProjection;{6f03e149-bfc9-4c01-b578-50338cec97fc})");
}
unsafe impl ::windows::core::Interface for Matrix3DProjection {
    type Vtable = IMatrix3DProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f03e149_bfc9_4c01_b578_50338cec97fc);
}
impl ::windows::core::RuntimeName for Matrix3DProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Matrix3DProjection";
}
impl ::core::convert::From<Matrix3DProjection> for ::windows::core::IUnknown {
    fn from(value: Matrix3DProjection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Matrix3DProjection> for ::windows::core::IUnknown {
    fn from(value: &Matrix3DProjection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Matrix3DProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Matrix3DProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Matrix3DProjection> for ::windows::core::IInspectable {
    fn from(value: Matrix3DProjection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Matrix3DProjection> for ::windows::core::IInspectable {
    fn from(value: &Matrix3DProjection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Matrix3DProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Matrix3DProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Matrix3DProjection> for Projection {
    fn from(value: Matrix3DProjection) -> Self {
        ::core::convert::Into::<Projection>::into(&value)
    }
}
impl ::core::convert::From<&Matrix3DProjection> for Projection {
    fn from(value: &Matrix3DProjection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Projection> for Matrix3DProjection {
    fn into_param(self) -> ::windows::core::Param<'a, Projection> {
        ::windows::core::Param::Owned(::core::convert::Into::<Projection>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Projection> for &Matrix3DProjection {
    fn into_param(self) -> ::windows::core::Param<'a, Projection> {
        ::windows::core::Param::Owned(::core::convert::Into::<Projection>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Matrix3DProjection> for super::DependencyObject {
    fn from(value: Matrix3DProjection) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Matrix3DProjection> for super::DependencyObject {
    fn from(value: &Matrix3DProjection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Matrix3DProjection {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Matrix3DProjection {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Matrix3DProjection {}
unsafe impl ::core::marker::Sync for Matrix3DProjection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MatrixHelper(pub ::windows::core::IInspectable);
impl MatrixHelper {
    pub fn Identity() -> ::windows::core::Result<Matrix> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__: Matrix = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Matrix>(result__)
        })
    }
    pub fn FromElements(m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64) -> ::windows::core::Result<Matrix> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__: Matrix = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), m11, m12, m21, m22, offsetx, offsety, &mut result__).from_abi::<Matrix>(result__)
        })
    }
    pub fn GetIsIdentity<'a, Param0: ::windows::core::IntoParam<'a, Matrix>>(target: Param0) -> ::windows::core::Result<bool> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Transform<'a, Param0: ::windows::core::IntoParam<'a, Matrix>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(target: Param0, point: Param1) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), target.into_param().abi(), point.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        })
    }
    pub fn IMatrixHelperStatics<R, F: FnOnce(&IMatrixHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MatrixHelper, IMatrixHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MatrixHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.MatrixHelper;{f3cf4882-06b5-48c8-9eb2-1763e9364038})");
}
unsafe impl ::windows::core::Interface for MatrixHelper {
    type Vtable = IMatrixHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3cf4882_06b5_48c8_9eb2_1763e9364038);
}
impl ::windows::core::RuntimeName for MatrixHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MatrixHelper";
}
impl ::core::convert::From<MatrixHelper> for ::windows::core::IUnknown {
    fn from(value: MatrixHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MatrixHelper> for ::windows::core::IUnknown {
    fn from(value: &MatrixHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MatrixHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MatrixHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MatrixHelper> for ::windows::core::IInspectable {
    fn from(value: MatrixHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MatrixHelper> for ::windows::core::IInspectable {
    fn from(value: &MatrixHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MatrixHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MatrixHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MatrixHelper {}
unsafe impl ::core::marker::Sync for MatrixHelper {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MatrixTransform(pub ::windows::core::IInspectable);
impl MatrixTransform {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MatrixTransform, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Matrix(&self) -> ::windows::core::Result<Matrix> {
        let this = self;
        unsafe {
            let mut result__: Matrix = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Matrix>(result__)
        }
    }
    pub fn SetMatrix<'a, Param0: ::windows::core::IntoParam<'a, Matrix>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn MatrixProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IMatrixTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IMatrixTransformStatics<R, F: FnOnce(&IMatrixTransformStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MatrixTransform, IMatrixTransformStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MatrixTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.MatrixTransform;{edfdd551-5fed-45fc-ae62-92a4b6cf9707})");
}
unsafe impl ::windows::core::Interface for MatrixTransform {
    type Vtable = IMatrixTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedfdd551_5fed_45fc_ae62_92a4b6cf9707);
}
impl ::windows::core::RuntimeName for MatrixTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MatrixTransform";
}
impl ::core::convert::From<MatrixTransform> for ::windows::core::IUnknown {
    fn from(value: MatrixTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MatrixTransform> for ::windows::core::IUnknown {
    fn from(value: &MatrixTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MatrixTransform> for ::windows::core::IInspectable {
    fn from(value: MatrixTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MatrixTransform> for ::windows::core::IInspectable {
    fn from(value: &MatrixTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MatrixTransform> for Transform {
    fn from(value: MatrixTransform) -> Self {
        ::core::convert::Into::<Transform>::into(&value)
    }
}
impl ::core::convert::From<&MatrixTransform> for Transform {
    fn from(value: &MatrixTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for &MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<MatrixTransform> for GeneralTransform {
    fn from(value: MatrixTransform) -> Self {
        ::core::convert::Into::<GeneralTransform>::into(&value)
    }
}
impl ::core::convert::From<&MatrixTransform> for GeneralTransform {
    fn from(value: &MatrixTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for &MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<MatrixTransform> for super::DependencyObject {
    fn from(value: MatrixTransform) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&MatrixTransform> for super::DependencyObject {
    fn from(value: &MatrixTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &MatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for MatrixTransform {}
unsafe impl ::core::marker::Sync for MatrixTransform {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCanPlayResponse(pub i32);
impl MediaCanPlayResponse {
    pub const NotSupported: MediaCanPlayResponse = MediaCanPlayResponse(0i32);
    pub const Maybe: MediaCanPlayResponse = MediaCanPlayResponse(1i32);
    pub const Probably: MediaCanPlayResponse = MediaCanPlayResponse(2i32);
}
impl ::core::convert::From<i32> for MediaCanPlayResponse {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCanPlayResponse {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCanPlayResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.MediaCanPlayResponse;i4)");
}
impl ::windows::core::DefaultType for MediaCanPlayResponse {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaElementState(pub i32);
impl MediaElementState {
    pub const Closed: MediaElementState = MediaElementState(0i32);
    pub const Opening: MediaElementState = MediaElementState(1i32);
    pub const Buffering: MediaElementState = MediaElementState(2i32);
    pub const Playing: MediaElementState = MediaElementState(3i32);
    pub const Paused: MediaElementState = MediaElementState(4i32);
    pub const Stopped: MediaElementState = MediaElementState(5i32);
}
impl ::core::convert::From<i32> for MediaElementState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaElementState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaElementState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.MediaElementState;i4)");
}
impl ::windows::core::DefaultType for MediaElementState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaTransportControlsThumbnailRequestedEventArgs(pub ::windows::core::IInspectable);
impl MediaTransportControlsThumbnailRequestedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnailImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaTransportControlsThumbnailRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.MediaTransportControlsThumbnailRequestedEventArgs;{e4a8b21c-e3c2-485c-ae69-f1537b76755a})");
}
unsafe impl ::windows::core::Interface for MediaTransportControlsThumbnailRequestedEventArgs {
    type Vtable = IMediaTransportControlsThumbnailRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4a8b21c_e3c2_485c_ae69_f1537b76755a);
}
impl ::windows::core::RuntimeName for MediaTransportControlsThumbnailRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MediaTransportControlsThumbnailRequestedEventArgs";
}
impl ::core::convert::From<MediaTransportControlsThumbnailRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaTransportControlsThumbnailRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaTransportControlsThumbnailRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaTransportControlsThumbnailRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaTransportControlsThumbnailRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaTransportControlsThumbnailRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaTransportControlsThumbnailRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaTransportControlsThumbnailRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaTransportControlsThumbnailRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaTransportControlsThumbnailRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaTransportControlsThumbnailRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaTransportControlsThumbnailRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaTransportControlsThumbnailRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaTransportControlsThumbnailRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PartialMediaFailureDetectedEventArgs(pub ::windows::core::IInspectable);
impl PartialMediaFailureDetectedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PartialMediaFailureDetectedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn StreamKind(&self) -> ::windows::core::Result<super::super::super::Media::Playback::FailedMediaStreamKind> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Media::Playback::FailedMediaStreamKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Media::Playback::FailedMediaStreamKind>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPartialMediaFailureDetectedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PartialMediaFailureDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PartialMediaFailureDetectedEventArgs;{02b65a91-e5a1-442b-88d3-2dc127bfc59b})");
}
unsafe impl ::windows::core::Interface for PartialMediaFailureDetectedEventArgs {
    type Vtable = IPartialMediaFailureDetectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b65a91_e5a1_442b_88d3_2dc127bfc59b);
}
impl ::windows::core::RuntimeName for PartialMediaFailureDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PartialMediaFailureDetectedEventArgs";
}
impl ::core::convert::From<PartialMediaFailureDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PartialMediaFailureDetectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PartialMediaFailureDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PartialMediaFailureDetectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PartialMediaFailureDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PartialMediaFailureDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PartialMediaFailureDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PartialMediaFailureDetectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PartialMediaFailureDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PartialMediaFailureDetectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PartialMediaFailureDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PartialMediaFailureDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PartialMediaFailureDetectedEventArgs {}
unsafe impl ::core::marker::Sync for PartialMediaFailureDetectedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PathFigure(pub ::windows::core::IInspectable);
impl PathFigure {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PathFigure, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Segments(&self) -> ::windows::core::Result<PathSegmentCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PathSegmentCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetSegments<'a, Param0: ::windows::core::IntoParam<'a, PathSegmentCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsClosed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsClosed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsFilled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsFilled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SegmentsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StartPointProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsClosedProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsFilledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPathFigureStatics<R, F: FnOnce(&IPathFigureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PathFigure, IPathFigureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PathFigure {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathFigure;{5d955c8c-5fa9-4dda-a3cc-10fcdcaa20d7})");
}
unsafe impl ::windows::core::Interface for PathFigure {
    type Vtable = IPathFigure_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d955c8c_5fa9_4dda_a3cc_10fcdcaa20d7);
}
impl ::windows::core::RuntimeName for PathFigure {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathFigure";
}
impl ::core::convert::From<PathFigure> for ::windows::core::IUnknown {
    fn from(value: PathFigure) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PathFigure> for ::windows::core::IUnknown {
    fn from(value: &PathFigure) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PathFigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PathFigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PathFigure> for ::windows::core::IInspectable {
    fn from(value: PathFigure) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PathFigure> for ::windows::core::IInspectable {
    fn from(value: &PathFigure) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PathFigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PathFigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PathFigure> for super::DependencyObject {
    fn from(value: PathFigure) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PathFigure> for super::DependencyObject {
    fn from(value: &PathFigure) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PathFigure {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PathFigure {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PathFigure {}
unsafe impl ::core::marker::Sync for PathFigure {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PathFigureCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PathFigureCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PathFigureCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<PathFigure> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<PathFigure>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PathFigure>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PathFigure>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, PathFigure>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, PathFigure>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, PathFigure>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, PathFigure>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<PathFigure as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<PathFigure as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<PathFigure>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<PathFigure>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<PathFigure>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PathFigureCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathFigureCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.PathFigure;{5d955c8c-5fa9-4dda-a3cc-10fcdcaa20d7})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PathFigureCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<PathFigure>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<PathFigure> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PathFigureCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathFigureCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PathFigureCollection> for ::windows::core::IUnknown {
    fn from(value: PathFigureCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PathFigureCollection> for ::windows::core::IUnknown {
    fn from(value: &PathFigureCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PathFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PathFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PathFigureCollection> for ::windows::core::IInspectable {
    fn from(value: PathFigureCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PathFigureCollection> for ::windows::core::IInspectable {
    fn from(value: &PathFigureCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PathFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PathFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PathFigureCollection> for super::super::super::Foundation::Collections::IVector<PathFigure> {
    fn from(value: PathFigureCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PathFigureCollection> for super::super::super::Foundation::Collections::IVector<PathFigure> {
    fn from(value: &PathFigureCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<PathFigure>> for PathFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<PathFigure>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<PathFigure>> for &PathFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<PathFigure>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PathFigureCollection> for super::super::super::Foundation::Collections::IIterable<PathFigure> {
    type Error = ::windows::core::Error;
    fn try_from(value: PathFigureCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PathFigureCollection> for super::super::super::Foundation::Collections::IIterable<PathFigure> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PathFigureCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PathFigure>> for PathFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<PathFigure>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PathFigure>> for &PathFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<PathFigure>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<PathFigure>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PathFigureCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PathFigureCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PathGeometry(pub ::windows::core::IInspectable);
impl PathGeometry {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PathGeometry, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FillRule(&self) -> ::windows::core::Result<FillRule> {
        let this = self;
        unsafe {
            let mut result__: FillRule = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FillRule>(result__)
        }
    }
    pub fn SetFillRule(&self, value: FillRule) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Figures(&self) -> ::windows::core::Result<PathFigureCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PathFigureCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetFigures<'a, Param0: ::windows::core::IntoParam<'a, PathFigureCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FillRuleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPathGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FiguresProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPathGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPathGeometryStatics<R, F: FnOnce(&IPathGeometryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PathGeometry, IPathGeometryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PathGeometry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathGeometry;{081b9df8-bae6-4bcb-813c-bde0e46dc8b7})");
}
unsafe impl ::windows::core::Interface for PathGeometry {
    type Vtable = IPathGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x081b9df8_bae6_4bcb_813c_bde0e46dc8b7);
}
impl ::windows::core::RuntimeName for PathGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathGeometry";
}
impl ::core::convert::From<PathGeometry> for ::windows::core::IUnknown {
    fn from(value: PathGeometry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PathGeometry> for ::windows::core::IUnknown {
    fn from(value: &PathGeometry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PathGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PathGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PathGeometry> for ::windows::core::IInspectable {
    fn from(value: PathGeometry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PathGeometry> for ::windows::core::IInspectable {
    fn from(value: &PathGeometry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PathGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PathGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PathGeometry> for Geometry {
    fn from(value: PathGeometry) -> Self {
        ::core::convert::Into::<Geometry>::into(&value)
    }
}
impl ::core::convert::From<&PathGeometry> for Geometry {
    fn from(value: &PathGeometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for PathGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for &PathGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PathGeometry> for super::DependencyObject {
    fn from(value: PathGeometry) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PathGeometry> for super::DependencyObject {
    fn from(value: &PathGeometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PathGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PathGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PathGeometry {}
unsafe impl ::core::marker::Sync for PathGeometry {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PathSegment(pub ::windows::core::IInspectable);
impl PathSegment {}
unsafe impl ::windows::core::RuntimeType for PathSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathSegment;{fcfa71cf-9ce3-474f-8157-10b6435a616b})");
}
unsafe impl ::windows::core::Interface for PathSegment {
    type Vtable = IPathSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcfa71cf_9ce3_474f_8157_10b6435a616b);
}
impl ::windows::core::RuntimeName for PathSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathSegment";
}
impl ::core::convert::From<PathSegment> for ::windows::core::IUnknown {
    fn from(value: PathSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PathSegment> for ::windows::core::IUnknown {
    fn from(value: &PathSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PathSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PathSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PathSegment> for ::windows::core::IInspectable {
    fn from(value: PathSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PathSegment> for ::windows::core::IInspectable {
    fn from(value: &PathSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PathSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PathSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PathSegment> for super::DependencyObject {
    fn from(value: PathSegment) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PathSegment> for super::DependencyObject {
    fn from(value: &PathSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PathSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PathSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PathSegment {}
unsafe impl ::core::marker::Sync for PathSegment {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PathSegmentCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PathSegmentCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PathSegmentCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<PathSegment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<PathSegment>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PathSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PathSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, PathSegment>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, PathSegment>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, PathSegment>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, PathSegment>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<PathSegment as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<PathSegment as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<PathSegment>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<PathSegment>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<PathSegment>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PathSegmentCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathSegmentCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.PathSegment;{fcfa71cf-9ce3-474f-8157-10b6435a616b})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PathSegmentCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<PathSegment>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<PathSegment> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PathSegmentCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathSegmentCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PathSegmentCollection> for ::windows::core::IUnknown {
    fn from(value: PathSegmentCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PathSegmentCollection> for ::windows::core::IUnknown {
    fn from(value: &PathSegmentCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PathSegmentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PathSegmentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PathSegmentCollection> for ::windows::core::IInspectable {
    fn from(value: PathSegmentCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PathSegmentCollection> for ::windows::core::IInspectable {
    fn from(value: &PathSegmentCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PathSegmentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PathSegmentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PathSegmentCollection> for super::super::super::Foundation::Collections::IVector<PathSegment> {
    fn from(value: PathSegmentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PathSegmentCollection> for super::super::super::Foundation::Collections::IVector<PathSegment> {
    fn from(value: &PathSegmentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<PathSegment>> for PathSegmentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<PathSegment>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<PathSegment>> for &PathSegmentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<PathSegment>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PathSegmentCollection> for super::super::super::Foundation::Collections::IIterable<PathSegment> {
    type Error = ::windows::core::Error;
    fn try_from(value: PathSegmentCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PathSegmentCollection> for super::super::super::Foundation::Collections::IIterable<PathSegment> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PathSegmentCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PathSegment>> for PathSegmentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<PathSegment>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PathSegment>> for &PathSegmentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<PathSegment>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<PathSegment>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PathSegmentCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PathSegmentCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PenLineCap(pub i32);
impl PenLineCap {
    pub const Flat: PenLineCap = PenLineCap(0i32);
    pub const Square: PenLineCap = PenLineCap(1i32);
    pub const Round: PenLineCap = PenLineCap(2i32);
    pub const Triangle: PenLineCap = PenLineCap(3i32);
}
impl ::core::convert::From<i32> for PenLineCap {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PenLineCap {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PenLineCap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.PenLineCap;i4)");
}
impl ::windows::core::DefaultType for PenLineCap {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PenLineJoin(pub i32);
impl PenLineJoin {
    pub const Miter: PenLineJoin = PenLineJoin(0i32);
    pub const Bevel: PenLineJoin = PenLineJoin(1i32);
    pub const Round: PenLineJoin = PenLineJoin(2i32);
}
impl ::core::convert::From<i32> for PenLineJoin {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PenLineJoin {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PenLineJoin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.PenLineJoin;i4)");
}
impl ::windows::core::DefaultType for PenLineJoin {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlaneProjection(pub ::windows::core::IInspectable);
impl PlaneProjection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlaneProjection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn LocalOffsetX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetLocalOffsetX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LocalOffsetY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetLocalOffsetY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LocalOffsetZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetLocalOffsetZ(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationZ(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterOfRotationX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterOfRotationX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterOfRotationY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterOfRotationY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterOfRotationZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterOfRotationZ(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GlobalOffsetX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGlobalOffsetX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GlobalOffsetY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGlobalOffsetY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GlobalOffsetZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGlobalOffsetZ(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn ProjectionMatrix(&self) -> ::windows::core::Result<Media3D::Matrix3D> {
        let this = self;
        unsafe {
            let mut result__: Media3D::Matrix3D = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Media3D::Matrix3D>(result__)
        }
    }
    pub fn LocalOffsetXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LocalOffsetYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LocalOffsetZProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationZProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterOfRotationXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterOfRotationYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterOfRotationZProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GlobalOffsetXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GlobalOffsetYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GlobalOffsetZProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ProjectionMatrixProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPlaneProjectionStatics<R, F: FnOnce(&IPlaneProjectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlaneProjection, IPlaneProjectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PlaneProjection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PlaneProjection;{e6f82bfa-6726-469a-b259-a5188347ca8f})");
}
unsafe impl ::windows::core::Interface for PlaneProjection {
    type Vtable = IPlaneProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6f82bfa_6726_469a_b259_a5188347ca8f);
}
impl ::windows::core::RuntimeName for PlaneProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PlaneProjection";
}
impl ::core::convert::From<PlaneProjection> for ::windows::core::IUnknown {
    fn from(value: PlaneProjection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlaneProjection> for ::windows::core::IUnknown {
    fn from(value: &PlaneProjection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaneProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlaneProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlaneProjection> for ::windows::core::IInspectable {
    fn from(value: PlaneProjection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlaneProjection> for ::windows::core::IInspectable {
    fn from(value: &PlaneProjection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaneProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlaneProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PlaneProjection> for Projection {
    fn from(value: PlaneProjection) -> Self {
        ::core::convert::Into::<Projection>::into(&value)
    }
}
impl ::core::convert::From<&PlaneProjection> for Projection {
    fn from(value: &PlaneProjection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Projection> for PlaneProjection {
    fn into_param(self) -> ::windows::core::Param<'a, Projection> {
        ::windows::core::Param::Owned(::core::convert::Into::<Projection>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Projection> for &PlaneProjection {
    fn into_param(self) -> ::windows::core::Param<'a, Projection> {
        ::windows::core::Param::Owned(::core::convert::Into::<Projection>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PlaneProjection> for super::DependencyObject {
    fn from(value: PlaneProjection) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PlaneProjection> for super::DependencyObject {
    fn from(value: &PlaneProjection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PlaneProjection {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PlaneProjection {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PlaneProjection {}
unsafe impl ::core::marker::Sync for PlaneProjection {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PointCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReplaceAll(&self, items: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Point>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PointCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PointCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};struct(Windows.Foundation.Point;f4;f4)))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PointCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<super::super::super::Foundation::Point>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Point> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PointCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PointCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PointCollection> for ::windows::core::IUnknown {
    fn from(value: PointCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PointCollection> for ::windows::core::IUnknown {
    fn from(value: &PointCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PointCollection> for ::windows::core::IInspectable {
    fn from(value: PointCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PointCollection> for ::windows::core::IInspectable {
    fn from(value: &PointCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PointCollection> for super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Point> {
    fn from(value: PointCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PointCollection> for super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Point> {
    fn from(value: &PointCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Point>> for PointCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Point>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Point>> for &PointCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Point>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PointCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point> {
    type Error = ::windows::core::Error;
    fn try_from(value: PointCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PointCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PointCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>> for PointCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>> for &PointCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PointCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PointCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PointCollection {
    type Item = super::super::super::Foundation::Point;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PointCollection {
    type Item = super::super::super::Foundation::Point;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PolyBezierSegment(pub ::windows::core::IInspectable);
impl PolyBezierSegment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PolyBezierSegment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Points(&self) -> ::windows::core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PointCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPoints<'a, Param0: ::windows::core::IntoParam<'a, PointCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PointsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolyBezierSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPolyBezierSegmentStatics<R, F: FnOnce(&IPolyBezierSegmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PolyBezierSegment, IPolyBezierSegmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PolyBezierSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PolyBezierSegment;{36805271-38c4-4bcf-96cd-028a6d38af25})");
}
unsafe impl ::windows::core::Interface for PolyBezierSegment {
    type Vtable = IPolyBezierSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36805271_38c4_4bcf_96cd_028a6d38af25);
}
impl ::windows::core::RuntimeName for PolyBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyBezierSegment";
}
impl ::core::convert::From<PolyBezierSegment> for ::windows::core::IUnknown {
    fn from(value: PolyBezierSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PolyBezierSegment> for ::windows::core::IUnknown {
    fn from(value: &PolyBezierSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PolyBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PolyBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PolyBezierSegment> for ::windows::core::IInspectable {
    fn from(value: PolyBezierSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PolyBezierSegment> for ::windows::core::IInspectable {
    fn from(value: &PolyBezierSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PolyBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PolyBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PolyBezierSegment> for PathSegment {
    fn from(value: PolyBezierSegment) -> Self {
        ::core::convert::Into::<PathSegment>::into(&value)
    }
}
impl ::core::convert::From<&PolyBezierSegment> for PathSegment {
    fn from(value: &PolyBezierSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for PolyBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for &PolyBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PolyBezierSegment> for super::DependencyObject {
    fn from(value: PolyBezierSegment) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PolyBezierSegment> for super::DependencyObject {
    fn from(value: &PolyBezierSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PolyBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PolyBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PolyBezierSegment {}
unsafe impl ::core::marker::Sync for PolyBezierSegment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PolyLineSegment(pub ::windows::core::IInspectable);
impl PolyLineSegment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PolyLineSegment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Points(&self) -> ::windows::core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PointCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPoints<'a, Param0: ::windows::core::IntoParam<'a, PointCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PointsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolyLineSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPolyLineSegmentStatics<R, F: FnOnce(&IPolyLineSegmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PolyLineSegment, IPolyLineSegmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PolyLineSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PolyLineSegment;{4b397f87-a2e6-479d-bdc8-6f4464646887})");
}
unsafe impl ::windows::core::Interface for PolyLineSegment {
    type Vtable = IPolyLineSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b397f87_a2e6_479d_bdc8_6f4464646887);
}
impl ::windows::core::RuntimeName for PolyLineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyLineSegment";
}
impl ::core::convert::From<PolyLineSegment> for ::windows::core::IUnknown {
    fn from(value: PolyLineSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PolyLineSegment> for ::windows::core::IUnknown {
    fn from(value: &PolyLineSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PolyLineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PolyLineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PolyLineSegment> for ::windows::core::IInspectable {
    fn from(value: PolyLineSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PolyLineSegment> for ::windows::core::IInspectable {
    fn from(value: &PolyLineSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PolyLineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PolyLineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PolyLineSegment> for PathSegment {
    fn from(value: PolyLineSegment) -> Self {
        ::core::convert::Into::<PathSegment>::into(&value)
    }
}
impl ::core::convert::From<&PolyLineSegment> for PathSegment {
    fn from(value: &PolyLineSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for PolyLineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for &PolyLineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PolyLineSegment> for super::DependencyObject {
    fn from(value: PolyLineSegment) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PolyLineSegment> for super::DependencyObject {
    fn from(value: &PolyLineSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PolyLineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PolyLineSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PolyLineSegment {}
unsafe impl ::core::marker::Sync for PolyLineSegment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PolyQuadraticBezierSegment(pub ::windows::core::IInspectable);
impl PolyQuadraticBezierSegment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PolyQuadraticBezierSegment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Points(&self) -> ::windows::core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PointCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPoints<'a, Param0: ::windows::core::IntoParam<'a, PointCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PointsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolyQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPolyQuadraticBezierSegmentStatics<R, F: FnOnce(&IPolyQuadraticBezierSegmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PolyQuadraticBezierSegment, IPolyQuadraticBezierSegmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PolyQuadraticBezierSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PolyQuadraticBezierSegment;{dd5ced7d-e6db-4c96-b6a1-3fce96e987a6})");
}
unsafe impl ::windows::core::Interface for PolyQuadraticBezierSegment {
    type Vtable = IPolyQuadraticBezierSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd5ced7d_e6db_4c96_b6a1_3fce96e987a6);
}
impl ::windows::core::RuntimeName for PolyQuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyQuadraticBezierSegment";
}
impl ::core::convert::From<PolyQuadraticBezierSegment> for ::windows::core::IUnknown {
    fn from(value: PolyQuadraticBezierSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PolyQuadraticBezierSegment> for ::windows::core::IUnknown {
    fn from(value: &PolyQuadraticBezierSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PolyQuadraticBezierSegment> for ::windows::core::IInspectable {
    fn from(value: PolyQuadraticBezierSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PolyQuadraticBezierSegment> for ::windows::core::IInspectable {
    fn from(value: &PolyQuadraticBezierSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PolyQuadraticBezierSegment> for PathSegment {
    fn from(value: PolyQuadraticBezierSegment) -> Self {
        ::core::convert::Into::<PathSegment>::into(&value)
    }
}
impl ::core::convert::From<&PolyQuadraticBezierSegment> for PathSegment {
    fn from(value: &PolyQuadraticBezierSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for &PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PolyQuadraticBezierSegment> for super::DependencyObject {
    fn from(value: PolyQuadraticBezierSegment) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PolyQuadraticBezierSegment> for super::DependencyObject {
    fn from(value: &PolyQuadraticBezierSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PolyQuadraticBezierSegment {}
unsafe impl ::core::marker::Sync for PolyQuadraticBezierSegment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Projection(pub ::windows::core::IInspectable);
impl Projection {}
unsafe impl ::windows::core::RuntimeType for Projection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Projection;{b3443557-7f39-4d04-a89c-844338cac897})");
}
unsafe impl ::windows::core::Interface for Projection {
    type Vtable = IProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3443557_7f39_4d04_a89c_844338cac897);
}
impl ::windows::core::RuntimeName for Projection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Projection";
}
impl ::core::convert::From<Projection> for ::windows::core::IUnknown {
    fn from(value: Projection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Projection> for ::windows::core::IUnknown {
    fn from(value: &Projection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Projection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Projection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Projection> for ::windows::core::IInspectable {
    fn from(value: Projection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Projection> for ::windows::core::IInspectable {
    fn from(value: &Projection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Projection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Projection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Projection> for super::DependencyObject {
    fn from(value: Projection) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Projection> for super::DependencyObject {
    fn from(value: &Projection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Projection {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Projection {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Projection {}
unsafe impl ::core::marker::Sync for Projection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct QuadraticBezierSegment(pub ::windows::core::IInspectable);
impl QuadraticBezierSegment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QuadraticBezierSegment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Point1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint1<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Point2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Point1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Point2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IQuadraticBezierSegmentStatics<R, F: FnOnce(&IQuadraticBezierSegmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QuadraticBezierSegment, IQuadraticBezierSegmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for QuadraticBezierSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.QuadraticBezierSegment;{2c509a5b-bf18-455a-a078-914b5232d8af})");
}
unsafe impl ::windows::core::Interface for QuadraticBezierSegment {
    type Vtable = IQuadraticBezierSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c509a5b_bf18_455a_a078_914b5232d8af);
}
impl ::windows::core::RuntimeName for QuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.QuadraticBezierSegment";
}
impl ::core::convert::From<QuadraticBezierSegment> for ::windows::core::IUnknown {
    fn from(value: QuadraticBezierSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&QuadraticBezierSegment> for ::windows::core::IUnknown {
    fn from(value: &QuadraticBezierSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for QuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a QuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<QuadraticBezierSegment> for ::windows::core::IInspectable {
    fn from(value: QuadraticBezierSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&QuadraticBezierSegment> for ::windows::core::IInspectable {
    fn from(value: &QuadraticBezierSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for QuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a QuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<QuadraticBezierSegment> for PathSegment {
    fn from(value: QuadraticBezierSegment) -> Self {
        ::core::convert::Into::<PathSegment>::into(&value)
    }
}
impl ::core::convert::From<&QuadraticBezierSegment> for PathSegment {
    fn from(value: &QuadraticBezierSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for QuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PathSegment> for &QuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, PathSegment> {
        ::windows::core::Param::Owned(::core::convert::Into::<PathSegment>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<QuadraticBezierSegment> for super::DependencyObject {
    fn from(value: QuadraticBezierSegment) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&QuadraticBezierSegment> for super::DependencyObject {
    fn from(value: &QuadraticBezierSegment) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for QuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &QuadraticBezierSegment {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for QuadraticBezierSegment {}
unsafe impl ::core::marker::Sync for QuadraticBezierSegment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RateChangedRoutedEventArgs(pub ::windows::core::IInspectable);
impl RateChangedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RateChangedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RateChangedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RateChangedRoutedEventArgs;{9016aa6f-3ca8-4c80-8e2f-8851a68f131f})");
}
unsafe impl ::windows::core::Interface for RateChangedRoutedEventArgs {
    type Vtable = IRateChangedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9016aa6f_3ca8_4c80_8e2f_8851a68f131f);
}
impl ::windows::core::RuntimeName for RateChangedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RateChangedRoutedEventArgs";
}
impl ::core::convert::From<RateChangedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RateChangedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RateChangedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RateChangedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RateChangedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RateChangedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RateChangedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RateChangedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RateChangedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: RateChangedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&RateChangedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &RateChangedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RateChangedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RateChangedRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RateChangedRoutedEventHandler(::windows::core::IUnknown);
impl RateChangedRoutedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RateChangedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = RateChangedRoutedEventHandler_box::<F> {
            vtable: &RateChangedRoutedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, RateChangedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RateChangedRoutedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({08e9a257-ae05-489b-8839-28c6225d2349})");
}
unsafe impl ::windows::core::Interface for RateChangedRoutedEventHandler {
    type Vtable = RateChangedRoutedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08e9a257_ae05_489b_8839_28c6225d2349);
}
#[repr(C)]
#[doc(hidden)]
pub struct RateChangedRoutedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct RateChangedRoutedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RateChangedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const RateChangedRoutedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RateChangedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> RateChangedRoutedEventHandler_box<F> {
    const VTABLE: RateChangedRoutedEventHandler_abi = RateChangedRoutedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RateChangedRoutedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
            &*(&e as *const <RateChangedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <RateChangedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RectangleGeometry(pub ::windows::core::IInspectable);
impl RectangleGeometry {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RectangleGeometry, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Rect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RectProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRectangleGeometryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IRectangleGeometryStatics<R, F: FnOnce(&IRectangleGeometryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RectangleGeometry, IRectangleGeometryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RectangleGeometry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RectangleGeometry;{a25a1f58-c575-4196-91cf-9fdfb10445c3})");
}
unsafe impl ::windows::core::Interface for RectangleGeometry {
    type Vtable = IRectangleGeometry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa25a1f58_c575_4196_91cf_9fdfb10445c3);
}
impl ::windows::core::RuntimeName for RectangleGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RectangleGeometry";
}
impl ::core::convert::From<RectangleGeometry> for ::windows::core::IUnknown {
    fn from(value: RectangleGeometry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RectangleGeometry> for ::windows::core::IUnknown {
    fn from(value: &RectangleGeometry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RectangleGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RectangleGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RectangleGeometry> for ::windows::core::IInspectable {
    fn from(value: RectangleGeometry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RectangleGeometry> for ::windows::core::IInspectable {
    fn from(value: &RectangleGeometry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RectangleGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RectangleGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RectangleGeometry> for Geometry {
    fn from(value: RectangleGeometry) -> Self {
        ::core::convert::Into::<Geometry>::into(&value)
    }
}
impl ::core::convert::From<&RectangleGeometry> for Geometry {
    fn from(value: &RectangleGeometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for RectangleGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Geometry> for &RectangleGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, Geometry> {
        ::windows::core::Param::Owned(::core::convert::Into::<Geometry>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RectangleGeometry> for super::DependencyObject {
    fn from(value: RectangleGeometry) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RectangleGeometry> for super::DependencyObject {
    fn from(value: &RectangleGeometry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for RectangleGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &RectangleGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RectangleGeometry {}
unsafe impl ::core::marker::Sync for RectangleGeometry {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RenderedEventArgs(pub ::windows::core::IInspectable);
impl RenderedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn FrameDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RenderedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RenderedEventArgs;{e349817d-81c7-4938-828c-a7e2797b35a6})");
}
unsafe impl ::windows::core::Interface for RenderedEventArgs {
    type Vtable = IRenderedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe349817d_81c7_4938_828c_a7e2797b35a6);
}
impl ::windows::core::RuntimeName for RenderedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RenderedEventArgs";
}
impl ::core::convert::From<RenderedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RenderedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RenderedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RenderedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RenderedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RenderedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RenderedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RenderedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RenderedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RenderedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RenderedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RenderedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RenderedEventArgs {}
unsafe impl ::core::marker::Sync for RenderedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RenderingEventArgs(pub ::windows::core::IInspectable);
impl RenderingEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn RenderingTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RenderingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RenderingEventArgs;{5bf7d30d-9748-4aed-8380-d7890eb776a0})");
}
unsafe impl ::windows::core::Interface for RenderingEventArgs {
    type Vtable = IRenderingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bf7d30d_9748_4aed_8380_d7890eb776a0);
}
impl ::windows::core::RuntimeName for RenderingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RenderingEventArgs";
}
impl ::core::convert::From<RenderingEventArgs> for ::windows::core::IUnknown {
    fn from(value: RenderingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RenderingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RenderingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RenderingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RenderingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RenderingEventArgs> for ::windows::core::IInspectable {
    fn from(value: RenderingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RenderingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RenderingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RenderingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RenderingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RenderingEventArgs {}
unsafe impl ::core::marker::Sync for RenderingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RevealBackgroundBrush(pub ::windows::core::IInspectable);
impl RevealBackgroundBrush {
    pub fn new() -> ::windows::core::Result<RevealBackgroundBrush> {
        Self::IRevealBackgroundBrushFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<RevealBackgroundBrush>(result__)
        })
    }
    pub fn IRevealBackgroundBrushFactory<R, F: FnOnce(&IRevealBackgroundBrushFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RevealBackgroundBrush, IRevealBackgroundBrushFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RevealBackgroundBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RevealBackgroundBrush;{261dcc0e-1991-4cdf-aee0-6350a3f90bb9})");
}
unsafe impl ::windows::core::Interface for RevealBackgroundBrush {
    type Vtable = IRevealBackgroundBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x261dcc0e_1991_4cdf_aee0_6350a3f90bb9);
}
impl ::windows::core::RuntimeName for RevealBackgroundBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBackgroundBrush";
}
impl ::core::convert::From<RevealBackgroundBrush> for ::windows::core::IUnknown {
    fn from(value: RevealBackgroundBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for ::windows::core::IUnknown {
    fn from(value: &RevealBackgroundBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for ::windows::core::IInspectable {
    fn from(value: RevealBackgroundBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for ::windows::core::IInspectable {
    fn from(value: &RevealBackgroundBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RevealBackgroundBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: RevealBackgroundBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RevealBackgroundBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &RevealBackgroundBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for RevealBrush {
    fn from(value: RevealBackgroundBrush) -> Self {
        ::core::convert::Into::<RevealBrush>::into(&value)
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for RevealBrush {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, RevealBrush> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, RevealBrush> {
        ::windows::core::Param::Owned(::core::convert::Into::<RevealBrush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, RevealBrush> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, RevealBrush> {
        ::windows::core::Param::Owned(::core::convert::Into::<RevealBrush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for XamlCompositionBrushBase {
    fn from(value: RevealBackgroundBrush) -> Self {
        ::core::convert::Into::<XamlCompositionBrushBase>::into(&value)
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for XamlCompositionBrushBase {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlCompositionBrushBase> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, XamlCompositionBrushBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlCompositionBrushBase> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, XamlCompositionBrushBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for Brush {
    fn from(value: RevealBackgroundBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for Brush {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for super::DependencyObject {
    fn from(value: RevealBackgroundBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for super::DependencyObject {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RevealBackgroundBrush {}
unsafe impl ::core::marker::Sync for RevealBackgroundBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RevealBorderBrush(pub ::windows::core::IInspectable);
impl RevealBorderBrush {
    pub fn new() -> ::windows::core::Result<RevealBorderBrush> {
        Self::IRevealBorderBrushFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<RevealBorderBrush>(result__)
        })
    }
    pub fn IRevealBorderBrushFactory<R, F: FnOnce(&IRevealBorderBrushFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RevealBorderBrush, IRevealBorderBrushFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RevealBorderBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RevealBorderBrush;{060ba115-c542-483c-8202-5f03331866c9})");
}
unsafe impl ::windows::core::Interface for RevealBorderBrush {
    type Vtable = IRevealBorderBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x060ba115_c542_483c_8202_5f03331866c9);
}
impl ::windows::core::RuntimeName for RevealBorderBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBorderBrush";
}
impl ::core::convert::From<RevealBorderBrush> for ::windows::core::IUnknown {
    fn from(value: RevealBorderBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RevealBorderBrush> for ::windows::core::IUnknown {
    fn from(value: &RevealBorderBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RevealBorderBrush> for ::windows::core::IInspectable {
    fn from(value: RevealBorderBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RevealBorderBrush> for ::windows::core::IInspectable {
    fn from(value: &RevealBorderBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RevealBorderBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: RevealBorderBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RevealBorderBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &RevealBorderBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<RevealBorderBrush> for RevealBrush {
    fn from(value: RevealBorderBrush) -> Self {
        ::core::convert::Into::<RevealBrush>::into(&value)
    }
}
impl ::core::convert::From<&RevealBorderBrush> for RevealBrush {
    fn from(value: &RevealBorderBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, RevealBrush> for RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, RevealBrush> {
        ::windows::core::Param::Owned(::core::convert::Into::<RevealBrush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, RevealBrush> for &RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, RevealBrush> {
        ::windows::core::Param::Owned(::core::convert::Into::<RevealBrush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RevealBorderBrush> for XamlCompositionBrushBase {
    fn from(value: RevealBorderBrush) -> Self {
        ::core::convert::Into::<XamlCompositionBrushBase>::into(&value)
    }
}
impl ::core::convert::From<&RevealBorderBrush> for XamlCompositionBrushBase {
    fn from(value: &RevealBorderBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlCompositionBrushBase> for RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, XamlCompositionBrushBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlCompositionBrushBase> for &RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, XamlCompositionBrushBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RevealBorderBrush> for Brush {
    fn from(value: RevealBorderBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&RevealBorderBrush> for Brush {
    fn from(value: &RevealBorderBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RevealBorderBrush> for super::DependencyObject {
    fn from(value: RevealBorderBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RevealBorderBrush> for super::DependencyObject {
    fn from(value: &RevealBorderBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &RevealBorderBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RevealBorderBrush {}
unsafe impl ::core::marker::Sync for RevealBorderBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RevealBrush(pub ::windows::core::IInspectable);
impl RevealBrush {
    pub fn Color(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetTheme(&self) -> ::windows::core::Result<super::ApplicationTheme> {
        let this = self;
        unsafe {
            let mut result__: super::ApplicationTheme = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ApplicationTheme>(result__)
        }
    }
    pub fn SetTargetTheme(&self, value: super::ApplicationTheme) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlwaysUseFallback(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysUseFallback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ColorProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TargetThemeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AlwaysUseFallbackProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StateProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SetState<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0, value: RevealBrushState) -> ::windows::core::Result<()> {
        Self::IRevealBrushStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    pub fn GetState<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<RevealBrushState> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__: RevealBrushState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<RevealBrushState>(result__)
        })
    }
    pub fn IRevealBrushStatics<R, F: FnOnce(&IRevealBrushStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RevealBrush, IRevealBrushStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RevealBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RevealBrush;{2036a0ed-8271-4398-9019-25872093f13f})");
}
unsafe impl ::windows::core::Interface for RevealBrush {
    type Vtable = IRevealBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2036a0ed_8271_4398_9019_25872093f13f);
}
impl ::windows::core::RuntimeName for RevealBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBrush";
}
impl ::core::convert::From<RevealBrush> for ::windows::core::IUnknown {
    fn from(value: RevealBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RevealBrush> for ::windows::core::IUnknown {
    fn from(value: &RevealBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RevealBrush> for ::windows::core::IInspectable {
    fn from(value: RevealBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RevealBrush> for ::windows::core::IInspectable {
    fn from(value: &RevealBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RevealBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: RevealBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RevealBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &RevealBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<RevealBrush> for XamlCompositionBrushBase {
    fn from(value: RevealBrush) -> Self {
        ::core::convert::Into::<XamlCompositionBrushBase>::into(&value)
    }
}
impl ::core::convert::From<&RevealBrush> for XamlCompositionBrushBase {
    fn from(value: &RevealBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlCompositionBrushBase> for RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, XamlCompositionBrushBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlCompositionBrushBase> for &RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, XamlCompositionBrushBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RevealBrush> for Brush {
    fn from(value: RevealBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&RevealBrush> for Brush {
    fn from(value: &RevealBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RevealBrush> for super::DependencyObject {
    fn from(value: RevealBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RevealBrush> for super::DependencyObject {
    fn from(value: &RevealBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &RevealBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RevealBrush {}
unsafe impl ::core::marker::Sync for RevealBrush {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RevealBrushState(pub i32);
impl RevealBrushState {
    pub const Normal: RevealBrushState = RevealBrushState(0i32);
    pub const PointerOver: RevealBrushState = RevealBrushState(1i32);
    pub const Pressed: RevealBrushState = RevealBrushState(2i32);
}
impl ::core::convert::From<i32> for RevealBrushState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RevealBrushState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RevealBrushState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.RevealBrushState;i4)");
}
impl ::windows::core::DefaultType for RevealBrushState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RotateTransform(pub ::windows::core::IInspectable);
impl RotateTransform {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RotateTransform, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Angle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetAngle(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AngleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IRotateTransformStatics<R, F: FnOnce(&IRotateTransformStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RotateTransform, IRotateTransformStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RotateTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RotateTransform;{688ea9b9-1e4e-4596-86e3-428b27334faf})");
}
unsafe impl ::windows::core::Interface for RotateTransform {
    type Vtable = IRotateTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x688ea9b9_1e4e_4596_86e3_428b27334faf);
}
impl ::windows::core::RuntimeName for RotateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RotateTransform";
}
impl ::core::convert::From<RotateTransform> for ::windows::core::IUnknown {
    fn from(value: RotateTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RotateTransform> for ::windows::core::IUnknown {
    fn from(value: &RotateTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RotateTransform> for ::windows::core::IInspectable {
    fn from(value: RotateTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RotateTransform> for ::windows::core::IInspectable {
    fn from(value: &RotateTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RotateTransform> for Transform {
    fn from(value: RotateTransform) -> Self {
        ::core::convert::Into::<Transform>::into(&value)
    }
}
impl ::core::convert::From<&RotateTransform> for Transform {
    fn from(value: &RotateTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for &RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RotateTransform> for GeneralTransform {
    fn from(value: RotateTransform) -> Self {
        ::core::convert::Into::<GeneralTransform>::into(&value)
    }
}
impl ::core::convert::From<&RotateTransform> for GeneralTransform {
    fn from(value: &RotateTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for &RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RotateTransform> for super::DependencyObject {
    fn from(value: RotateTransform) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RotateTransform> for super::DependencyObject {
    fn from(value: &RotateTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &RotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RotateTransform {}
unsafe impl ::core::marker::Sync for RotateTransform {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScaleTransform(pub ::windows::core::IInspectable);
impl ScaleTransform {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScaleTransform, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IScaleTransformStatics<R, F: FnOnce(&IScaleTransformStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScaleTransform, IScaleTransformStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ScaleTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ScaleTransform;{ed67f18d-936e-43ab-929a-e9cd0a511e52})");
}
unsafe impl ::windows::core::Interface for ScaleTransform {
    type Vtable = IScaleTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed67f18d_936e_43ab_929a_e9cd0a511e52);
}
impl ::windows::core::RuntimeName for ScaleTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ScaleTransform";
}
impl ::core::convert::From<ScaleTransform> for ::windows::core::IUnknown {
    fn from(value: ScaleTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScaleTransform> for ::windows::core::IUnknown {
    fn from(value: &ScaleTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScaleTransform> for ::windows::core::IInspectable {
    fn from(value: ScaleTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScaleTransform> for ::windows::core::IInspectable {
    fn from(value: &ScaleTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ScaleTransform> for Transform {
    fn from(value: ScaleTransform) -> Self {
        ::core::convert::Into::<Transform>::into(&value)
    }
}
impl ::core::convert::From<&ScaleTransform> for Transform {
    fn from(value: &ScaleTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for &ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ScaleTransform> for GeneralTransform {
    fn from(value: ScaleTransform) -> Self {
        ::core::convert::Into::<GeneralTransform>::into(&value)
    }
}
impl ::core::convert::From<&ScaleTransform> for GeneralTransform {
    fn from(value: &ScaleTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for &ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ScaleTransform> for super::DependencyObject {
    fn from(value: ScaleTransform) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ScaleTransform> for super::DependencyObject {
    fn from(value: &ScaleTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &ScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ScaleTransform {}
unsafe impl ::core::marker::Sync for ScaleTransform {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Shadow(pub ::windows::core::IInspectable);
impl Shadow {}
unsafe impl ::windows::core::RuntimeType for Shadow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Shadow;{6813a583-f3b4-5fcf-8694-2cd0aefc2fad})");
}
unsafe impl ::windows::core::Interface for Shadow {
    type Vtable = IShadow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6813a583_f3b4_5fcf_8694_2cd0aefc2fad);
}
impl ::windows::core::RuntimeName for Shadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Shadow";
}
impl ::core::convert::From<Shadow> for ::windows::core::IUnknown {
    fn from(value: Shadow) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Shadow> for ::windows::core::IUnknown {
    fn from(value: &Shadow) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Shadow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Shadow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Shadow> for ::windows::core::IInspectable {
    fn from(value: Shadow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Shadow> for ::windows::core::IInspectable {
    fn from(value: &Shadow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Shadow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Shadow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Shadow> for super::DependencyObject {
    fn from(value: Shadow) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Shadow> for super::DependencyObject {
    fn from(value: &Shadow) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Shadow {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Shadow {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Shadow {}
unsafe impl ::core::marker::Sync for Shadow {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SkewTransform(pub ::windows::core::IInspectable);
impl SkewTransform {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SkewTransform, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AngleX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetAngleX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AngleY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetAngleY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AngleXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AngleYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ISkewTransformStatics<R, F: FnOnce(&ISkewTransformStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SkewTransform, ISkewTransformStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SkewTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.SkewTransform;{4e8a3b15-7a0f-4617-9e98-1e65bdc92115})");
}
unsafe impl ::windows::core::Interface for SkewTransform {
    type Vtable = ISkewTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e8a3b15_7a0f_4617_9e98_1e65bdc92115);
}
impl ::windows::core::RuntimeName for SkewTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.SkewTransform";
}
impl ::core::convert::From<SkewTransform> for ::windows::core::IUnknown {
    fn from(value: SkewTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SkewTransform> for ::windows::core::IUnknown {
    fn from(value: &SkewTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SkewTransform> for ::windows::core::IInspectable {
    fn from(value: SkewTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SkewTransform> for ::windows::core::IInspectable {
    fn from(value: &SkewTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SkewTransform> for Transform {
    fn from(value: SkewTransform) -> Self {
        ::core::convert::Into::<Transform>::into(&value)
    }
}
impl ::core::convert::From<&SkewTransform> for Transform {
    fn from(value: &SkewTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for &SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SkewTransform> for GeneralTransform {
    fn from(value: SkewTransform) -> Self {
        ::core::convert::Into::<GeneralTransform>::into(&value)
    }
}
impl ::core::convert::From<&SkewTransform> for GeneralTransform {
    fn from(value: &SkewTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for &SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SkewTransform> for super::DependencyObject {
    fn from(value: SkewTransform) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SkewTransform> for super::DependencyObject {
    fn from(value: &SkewTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &SkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SkewTransform {}
unsafe impl ::core::marker::Sync for SkewTransform {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SolidColorBrush(pub ::windows::core::IInspectable);
impl SolidColorBrush {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SolidColorBrush, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Color(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CreateInstanceWithColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(color: Param0) -> ::windows::core::Result<SolidColorBrush> {
        Self::ISolidColorBrushFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), color.into_param().abi(), &mut result__).from_abi::<SolidColorBrush>(result__)
        })
    }
    pub fn ColorProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ISolidColorBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ISolidColorBrushFactory<R, F: FnOnce(&ISolidColorBrushFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SolidColorBrush, ISolidColorBrushFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISolidColorBrushStatics<R, F: FnOnce(&ISolidColorBrushStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SolidColorBrush, ISolidColorBrushStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SolidColorBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.SolidColorBrush;{9d850850-66f3-48df-9a8f-824bd5e070af})");
}
unsafe impl ::windows::core::Interface for SolidColorBrush {
    type Vtable = ISolidColorBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d850850_66f3_48df_9a8f_824bd5e070af);
}
impl ::windows::core::RuntimeName for SolidColorBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.SolidColorBrush";
}
impl ::core::convert::From<SolidColorBrush> for ::windows::core::IUnknown {
    fn from(value: SolidColorBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SolidColorBrush> for ::windows::core::IUnknown {
    fn from(value: &SolidColorBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SolidColorBrush> for ::windows::core::IInspectable {
    fn from(value: SolidColorBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SolidColorBrush> for ::windows::core::IInspectable {
    fn from(value: &SolidColorBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<SolidColorBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SolidColorBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&SolidColorBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SolidColorBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<SolidColorBrush> for Brush {
    fn from(value: SolidColorBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&SolidColorBrush> for Brush {
    fn from(value: &SolidColorBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SolidColorBrush> for super::DependencyObject {
    fn from(value: SolidColorBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SolidColorBrush> for super::DependencyObject {
    fn from(value: &SolidColorBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &SolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SolidColorBrush {}
unsafe impl ::core::marker::Sync for SolidColorBrush {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Stereo3DVideoPackingMode(pub i32);
impl Stereo3DVideoPackingMode {
    pub const None: Stereo3DVideoPackingMode = Stereo3DVideoPackingMode(0i32);
    pub const SideBySide: Stereo3DVideoPackingMode = Stereo3DVideoPackingMode(1i32);
    pub const TopBottom: Stereo3DVideoPackingMode = Stereo3DVideoPackingMode(2i32);
}
impl ::core::convert::From<i32> for Stereo3DVideoPackingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for Stereo3DVideoPackingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Stereo3DVideoPackingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Stereo3DVideoPackingMode;i4)");
}
impl ::windows::core::DefaultType for Stereo3DVideoPackingMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Stereo3DVideoRenderMode(pub i32);
impl Stereo3DVideoRenderMode {
    pub const Mono: Stereo3DVideoRenderMode = Stereo3DVideoRenderMode(0i32);
    pub const Stereo: Stereo3DVideoRenderMode = Stereo3DVideoRenderMode(1i32);
}
impl ::core::convert::From<i32> for Stereo3DVideoRenderMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for Stereo3DVideoRenderMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Stereo3DVideoRenderMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Stereo3DVideoRenderMode;i4)");
}
impl ::windows::core::DefaultType for Stereo3DVideoRenderMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Stretch = Stretch(0i32);
    pub const Fill: Stretch = Stretch(1i32);
    pub const Uniform: Stretch = Stretch(2i32);
    pub const UniformToFill: Stretch = Stretch(3i32);
}
impl ::core::convert::From<i32> for Stretch {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for Stretch {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Stretch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Stretch;i4)");
}
impl ::windows::core::DefaultType for Stretch {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StyleSimulations(pub i32);
impl StyleSimulations {
    pub const None: StyleSimulations = StyleSimulations(0i32);
    pub const BoldSimulation: StyleSimulations = StyleSimulations(1i32);
    pub const ItalicSimulation: StyleSimulations = StyleSimulations(2i32);
    pub const BoldItalicSimulation: StyleSimulations = StyleSimulations(3i32);
}
impl ::core::convert::From<i32> for StyleSimulations {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StyleSimulations {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StyleSimulations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.StyleSimulations;i4)");
}
impl ::windows::core::DefaultType for StyleSimulations {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SweepDirection(pub i32);
impl SweepDirection {
    pub const Counterclockwise: SweepDirection = SweepDirection(0i32);
    pub const Clockwise: SweepDirection = SweepDirection(1i32);
}
impl ::core::convert::From<i32> for SweepDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SweepDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SweepDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.SweepDirection;i4)");
}
impl ::windows::core::DefaultType for SweepDirection {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ThemeShadow(pub ::windows::core::IInspectable);
impl ThemeShadow {
    pub fn Receivers(&self) -> ::windows::core::Result<super::UIElementWeakCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElementWeakCollection>(result__)
        }
    }
    pub fn new() -> ::windows::core::Result<ThemeShadow> {
        Self::IThemeShadowFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ThemeShadow>(result__)
        })
    }
    pub fn IThemeShadowFactory<R, F: FnOnce(&IThemeShadowFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ThemeShadow, IThemeShadowFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ThemeShadow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ThemeShadow;{3eccad09-7985-5f39-8b62-6c10696dca6f})");
}
unsafe impl ::windows::core::Interface for ThemeShadow {
    type Vtable = IThemeShadow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3eccad09_7985_5f39_8b62_6c10696dca6f);
}
impl ::windows::core::RuntimeName for ThemeShadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ThemeShadow";
}
impl ::core::convert::From<ThemeShadow> for ::windows::core::IUnknown {
    fn from(value: ThemeShadow) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ThemeShadow> for ::windows::core::IUnknown {
    fn from(value: &ThemeShadow) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ThemeShadow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ThemeShadow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ThemeShadow> for ::windows::core::IInspectable {
    fn from(value: ThemeShadow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ThemeShadow> for ::windows::core::IInspectable {
    fn from(value: &ThemeShadow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ThemeShadow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ThemeShadow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ThemeShadow> for Shadow {
    fn from(value: ThemeShadow) -> Self {
        ::core::convert::Into::<Shadow>::into(&value)
    }
}
impl ::core::convert::From<&ThemeShadow> for Shadow {
    fn from(value: &ThemeShadow) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shadow> for ThemeShadow {
    fn into_param(self) -> ::windows::core::Param<'a, Shadow> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shadow>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shadow> for &ThemeShadow {
    fn into_param(self) -> ::windows::core::Param<'a, Shadow> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shadow>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ThemeShadow> for super::DependencyObject {
    fn from(value: ThemeShadow) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ThemeShadow> for super::DependencyObject {
    fn from(value: &ThemeShadow) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for ThemeShadow {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &ThemeShadow {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ThemeShadow {}
unsafe impl ::core::marker::Sync for ThemeShadow {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileBrush(pub ::windows::core::IInspectable);
impl TileBrush {
    pub fn AlignmentX(&self) -> ::windows::core::Result<AlignmentX> {
        let this = self;
        unsafe {
            let mut result__: AlignmentX = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AlignmentX>(result__)
        }
    }
    pub fn SetAlignmentX(&self, value: AlignmentX) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlignmentY(&self) -> ::windows::core::Result<AlignmentY> {
        let this = self;
        unsafe {
            let mut result__: AlignmentY = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AlignmentY>(result__)
        }
    }
    pub fn SetAlignmentY(&self, value: AlignmentY) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Stretch(&self) -> ::windows::core::Result<Stretch> {
        let this = self;
        unsafe {
            let mut result__: Stretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Stretch>(result__)
        }
    }
    pub fn SetStretch(&self, value: Stretch) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlignmentXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AlignmentYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StretchProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ITileBrushStatics<R, F: FnOnce(&ITileBrushStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TileBrush, ITileBrushStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TileBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TileBrush;{c201cf06-cd84-48a5-9607-664d7361cd61})");
}
unsafe impl ::windows::core::Interface for TileBrush {
    type Vtable = ITileBrush_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc201cf06_cd84_48a5_9607_664d7361cd61);
}
impl ::windows::core::RuntimeName for TileBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TileBrush";
}
impl ::core::convert::From<TileBrush> for ::windows::core::IUnknown {
    fn from(value: TileBrush) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileBrush> for ::windows::core::IUnknown {
    fn from(value: &TileBrush) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileBrush> for ::windows::core::IInspectable {
    fn from(value: TileBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileBrush> for ::windows::core::IInspectable {
    fn from(value: &TileBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<TileBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: TileBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&TileBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &TileBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<TileBrush> for Brush {
    fn from(value: TileBrush) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&TileBrush> for Brush {
    fn from(value: &TileBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<TileBrush> for super::DependencyObject {
    fn from(value: TileBrush) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TileBrush> for super::DependencyObject {
    fn from(value: &TileBrush) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for TileBrush {}
unsafe impl ::core::marker::Sync for TileBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TimelineMarker(pub ::windows::core::IInspectable);
impl TimelineMarker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimelineMarker, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Time(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TimeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITimelineMarkerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITimelineMarkerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TextProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITimelineMarkerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ITimelineMarkerStatics<R, F: FnOnce(&ITimelineMarkerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimelineMarker, ITimelineMarkerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TimelineMarker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TimelineMarker;{a68ef02d-45ba-4e50-8cad-aaea3a227af5})");
}
unsafe impl ::windows::core::Interface for TimelineMarker {
    type Vtable = ITimelineMarker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa68ef02d_45ba_4e50_8cad_aaea3a227af5);
}
impl ::windows::core::RuntimeName for TimelineMarker {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarker";
}
impl ::core::convert::From<TimelineMarker> for ::windows::core::IUnknown {
    fn from(value: TimelineMarker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TimelineMarker> for ::windows::core::IUnknown {
    fn from(value: &TimelineMarker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimelineMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TimelineMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TimelineMarker> for ::windows::core::IInspectable {
    fn from(value: TimelineMarker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TimelineMarker> for ::windows::core::IInspectable {
    fn from(value: &TimelineMarker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimelineMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TimelineMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TimelineMarker> for super::DependencyObject {
    fn from(value: TimelineMarker) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TimelineMarker> for super::DependencyObject {
    fn from(value: &TimelineMarker) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TimelineMarker {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TimelineMarker {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for TimelineMarker {}
unsafe impl ::core::marker::Sync for TimelineMarker {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TimelineMarkerCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl TimelineMarkerCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimelineMarkerCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<TimelineMarker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<TimelineMarker>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<TimelineMarker>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<TimelineMarker>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, TimelineMarker>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, TimelineMarker>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, TimelineMarker>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, TimelineMarker>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<TimelineMarker as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<TimelineMarker as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<TimelineMarker>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<TimelineMarker>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<TimelineMarker>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for TimelineMarkerCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TimelineMarkerCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.TimelineMarker;{a68ef02d-45ba-4e50-8cad-aaea3a227af5})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for TimelineMarkerCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<TimelineMarker>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<TimelineMarker> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for TimelineMarkerCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarkerCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TimelineMarkerCollection> for ::windows::core::IUnknown {
    fn from(value: TimelineMarkerCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TimelineMarkerCollection> for ::windows::core::IUnknown {
    fn from(value: &TimelineMarkerCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimelineMarkerCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TimelineMarkerCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TimelineMarkerCollection> for ::windows::core::IInspectable {
    fn from(value: TimelineMarkerCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TimelineMarkerCollection> for ::windows::core::IInspectable {
    fn from(value: &TimelineMarkerCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimelineMarkerCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TimelineMarkerCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TimelineMarkerCollection> for super::super::super::Foundation::Collections::IVector<TimelineMarker> {
    fn from(value: TimelineMarkerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TimelineMarkerCollection> for super::super::super::Foundation::Collections::IVector<TimelineMarker> {
    fn from(value: &TimelineMarkerCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<TimelineMarker>> for TimelineMarkerCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<TimelineMarker>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<TimelineMarker>> for &TimelineMarkerCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<TimelineMarker>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<TimelineMarkerCollection> for super::super::super::Foundation::Collections::IIterable<TimelineMarker> {
    type Error = ::windows::core::Error;
    fn try_from(value: TimelineMarkerCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&TimelineMarkerCollection> for super::super::super::Foundation::Collections::IIterable<TimelineMarker> {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimelineMarkerCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<TimelineMarker>> for TimelineMarkerCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<TimelineMarker>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<TimelineMarker>> for &TimelineMarkerCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<TimelineMarker>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<TimelineMarker>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for TimelineMarkerCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for TimelineMarkerCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for TimelineMarkerCollection {
    type Item = TimelineMarker;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &TimelineMarkerCollection {
    type Item = TimelineMarker;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TimelineMarkerRoutedEventArgs(pub ::windows::core::IInspectable);
impl TimelineMarkerRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimelineMarkerRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Marker(&self) -> ::windows::core::Result<TimelineMarker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimelineMarker>(result__)
        }
    }
    pub fn SetMarker<'a, Param0: ::windows::core::IntoParam<'a, TimelineMarker>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TimelineMarkerRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TimelineMarkerRoutedEventArgs;{7c3b3ef3-2c88-4d9c-99b6-46cdbd48d4c1})");
}
unsafe impl ::windows::core::Interface for TimelineMarkerRoutedEventArgs {
    type Vtable = ITimelineMarkerRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c3b3ef3_2c88_4d9c_99b6_46cdbd48d4c1);
}
impl ::windows::core::RuntimeName for TimelineMarkerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarkerRoutedEventArgs";
}
impl ::core::convert::From<TimelineMarkerRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TimelineMarkerRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TimelineMarkerRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TimelineMarkerRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TimelineMarkerRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TimelineMarkerRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TimelineMarkerRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TimelineMarkerRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TimelineMarkerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: TimelineMarkerRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&TimelineMarkerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &TimelineMarkerRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for TimelineMarkerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TimelineMarkerRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TimelineMarkerRoutedEventHandler(::windows::core::IUnknown);
impl TimelineMarkerRoutedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TimelineMarkerRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = TimelineMarkerRoutedEventHandler_box::<F> {
            vtable: &TimelineMarkerRoutedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, TimelineMarkerRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TimelineMarkerRoutedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({72e2fa9c-6dea-4cbe-a159-06ce95fbeced})");
}
unsafe impl ::windows::core::Interface for TimelineMarkerRoutedEventHandler {
    type Vtable = TimelineMarkerRoutedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72e2fa9c_6dea_4cbe_a159_06ce95fbeced);
}
#[repr(C)]
#[doc(hidden)]
pub struct TimelineMarkerRoutedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct TimelineMarkerRoutedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TimelineMarkerRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const TimelineMarkerRoutedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TimelineMarkerRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> TimelineMarkerRoutedEventHandler_box<F> {
    const VTABLE: TimelineMarkerRoutedEventHandler_abi = TimelineMarkerRoutedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<TimelineMarkerRoutedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
            &*(&e as *const <TimelineMarkerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <TimelineMarkerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Transform(pub ::windows::core::IInspectable);
impl Transform {}
unsafe impl ::windows::core::RuntimeType for Transform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Transform;{4df74078-bfd6-4ed1-9682-d2fd8bf2fe6f})");
}
unsafe impl ::windows::core::Interface for Transform {
    type Vtable = ITransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4df74078_bfd6_4ed1_9682_d2fd8bf2fe6f);
}
impl ::windows::core::RuntimeName for Transform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Transform";
}
impl ::core::convert::From<Transform> for ::windows::core::IUnknown {
    fn from(value: Transform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Transform> for ::windows::core::IUnknown {
    fn from(value: &Transform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Transform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Transform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Transform> for ::windows::core::IInspectable {
    fn from(value: Transform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Transform> for ::windows::core::IInspectable {
    fn from(value: &Transform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Transform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Transform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Transform> for GeneralTransform {
    fn from(value: Transform) -> Self {
        ::core::convert::Into::<GeneralTransform>::into(&value)
    }
}
impl ::core::convert::From<&Transform> for GeneralTransform {
    fn from(value: &Transform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for Transform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for &Transform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Transform> for super::DependencyObject {
    fn from(value: Transform) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Transform> for super::DependencyObject {
    fn from(value: &Transform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Transform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Transform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Transform {}
unsafe impl ::core::marker::Sync for Transform {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TransformCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl TransformCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransformCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Transform>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Transform>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Transform>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Transform>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Transform>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Transform>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Transform>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Transform as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Transform as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<Transform>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<Transform>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<Transform>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for TransformCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TransformCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Transform;{4df74078-bfd6-4ed1-9682-d2fd8bf2fe6f})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for TransformCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<Transform>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<Transform> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for TransformCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TransformCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TransformCollection> for ::windows::core::IUnknown {
    fn from(value: TransformCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TransformCollection> for ::windows::core::IUnknown {
    fn from(value: &TransformCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransformCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TransformCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TransformCollection> for ::windows::core::IInspectable {
    fn from(value: TransformCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TransformCollection> for ::windows::core::IInspectable {
    fn from(value: &TransformCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TransformCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TransformCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TransformCollection> for super::super::super::Foundation::Collections::IVector<Transform> {
    fn from(value: TransformCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TransformCollection> for super::super::super::Foundation::Collections::IVector<Transform> {
    fn from(value: &TransformCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Transform>> for TransformCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Transform>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Transform>> for &TransformCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Transform>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<TransformCollection> for super::super::super::Foundation::Collections::IIterable<Transform> {
    type Error = ::windows::core::Error;
    fn try_from(value: TransformCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&TransformCollection> for super::super::super::Foundation::Collections::IIterable<Transform> {
    type Error = ::windows::core::Error;
    fn try_from(value: &TransformCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Transform>> for TransformCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Transform>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Transform>> for &TransformCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Transform>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<Transform>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for TransformCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for TransformCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for TransformCollection {
    type Item = Transform;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &TransformCollection {
    type Item = Transform;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TransformGroup(pub ::windows::core::IInspectable);
impl TransformGroup {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransformGroup, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<TransformCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TransformCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetChildren<'a, Param0: ::windows::core::IntoParam<'a, TransformCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<Matrix> {
        let this = self;
        unsafe {
            let mut result__: Matrix = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Matrix>(result__)
        }
    }
    pub fn ChildrenProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITransformGroupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ITransformGroupStatics<R, F: FnOnce(&ITransformGroupStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransformGroup, ITransformGroupStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TransformGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TransformGroup;{63418ccc-8d2d-4737-b951-2afce1ddc4c4})");
}
unsafe impl ::windows::core::Interface for TransformGroup {
    type Vtable = ITransformGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63418ccc_8d2d_4737_b951_2afce1ddc4c4);
}
impl ::windows::core::RuntimeName for TransformGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TransformGroup";
}
impl ::core::convert::From<TransformGroup> for ::windows::core::IUnknown {
    fn from(value: TransformGroup) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TransformGroup> for ::windows::core::IUnknown {
    fn from(value: &TransformGroup) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TransformGroup> for ::windows::core::IInspectable {
    fn from(value: TransformGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TransformGroup> for ::windows::core::IInspectable {
    fn from(value: &TransformGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TransformGroup> for Transform {
    fn from(value: TransformGroup) -> Self {
        ::core::convert::Into::<Transform>::into(&value)
    }
}
impl ::core::convert::From<&TransformGroup> for Transform {
    fn from(value: &TransformGroup) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for &TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<TransformGroup> for GeneralTransform {
    fn from(value: TransformGroup) -> Self {
        ::core::convert::Into::<GeneralTransform>::into(&value)
    }
}
impl ::core::convert::From<&TransformGroup> for GeneralTransform {
    fn from(value: &TransformGroup) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for &TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<TransformGroup> for super::DependencyObject {
    fn from(value: TransformGroup) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TransformGroup> for super::DependencyObject {
    fn from(value: &TransformGroup) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TransformGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for TransformGroup {}
unsafe impl ::core::marker::Sync for TransformGroup {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TranslateTransform(pub ::windows::core::IInspectable);
impl TranslateTransform {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TranslateTransform, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn X(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Y(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn XProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITranslateTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn YProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITranslateTransformStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ITranslateTransformStatics<R, F: FnOnce(&ITranslateTransformStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TranslateTransform, ITranslateTransformStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TranslateTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TranslateTransform;{c975905c-3c36-4229-817b-178f64c0e113})");
}
unsafe impl ::windows::core::Interface for TranslateTransform {
    type Vtable = ITranslateTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc975905c_3c36_4229_817b_178f64c0e113);
}
impl ::windows::core::RuntimeName for TranslateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TranslateTransform";
}
impl ::core::convert::From<TranslateTransform> for ::windows::core::IUnknown {
    fn from(value: TranslateTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TranslateTransform> for ::windows::core::IUnknown {
    fn from(value: &TranslateTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TranslateTransform> for ::windows::core::IInspectable {
    fn from(value: TranslateTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TranslateTransform> for ::windows::core::IInspectable {
    fn from(value: &TranslateTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TranslateTransform> for Transform {
    fn from(value: TranslateTransform) -> Self {
        ::core::convert::Into::<Transform>::into(&value)
    }
}
impl ::core::convert::From<&TranslateTransform> for Transform {
    fn from(value: &TranslateTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform> for &TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, Transform> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<TranslateTransform> for GeneralTransform {
    fn from(value: TranslateTransform) -> Self {
        ::core::convert::Into::<GeneralTransform>::into(&value)
    }
}
impl ::core::convert::From<&TranslateTransform> for GeneralTransform {
    fn from(value: &TranslateTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, GeneralTransform> for &TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, GeneralTransform> {
        ::windows::core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<TranslateTransform> for super::DependencyObject {
    fn from(value: TranslateTransform) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TranslateTransform> for super::DependencyObject {
    fn from(value: &TranslateTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for TranslateTransform {}
unsafe impl ::core::marker::Sync for TranslateTransform {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VisualTreeHelper(pub ::windows::core::IInspectable);
impl VisualTreeHelper {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindElementsInHostCoordinatesPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(intersectingpoint: Param0, subtree: Param1) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), intersectingpoint.into_param().abi(), subtree.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::UIElement>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindElementsInHostCoordinatesRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(intersectingrect: Param0, subtree: Param1) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), intersectingrect.into_param().abi(), subtree.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::UIElement>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllElementsInHostCoordinatesPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(intersectingpoint: Param0, subtree: Param1, includeallelements: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), intersectingpoint.into_param().abi(), subtree.into_param().abi(), includeallelements, &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::UIElement>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllElementsInHostCoordinatesRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(intersectingrect: Param0, subtree: Param1, includeallelements: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), intersectingrect.into_param().abi(), subtree.into_param().abi(), includeallelements, &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::UIElement>>(result__)
        })
    }
    pub fn GetChild<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(reference: Param0, childindex: i32) -> ::windows::core::Result<super::DependencyObject> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), reference.into_param().abi(), childindex, &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn GetChildrenCount<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(reference: Param0) -> ::windows::core::Result<i32> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), reference.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn GetParent<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(reference: Param0) -> ::windows::core::Result<super::DependencyObject> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), reference.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn DisconnectChildrenRecursive<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<()> {
        Self::IVisualTreeHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), element.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives"))]
    pub fn GetOpenPopups<'a, Param0: ::windows::core::IntoParam<'a, super::Window>>(window: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>> {
        Self::IVisualTreeHelperStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), window.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives"))]
    pub fn GetOpenPopupsForXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(xamlroot: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>> {
        Self::IVisualTreeHelperStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xamlroot.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>(result__)
        })
    }
    pub fn IVisualTreeHelperStatics<R, F: FnOnce(&IVisualTreeHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VisualTreeHelper, IVisualTreeHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualTreeHelperStatics2<R, F: FnOnce(&IVisualTreeHelperStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VisualTreeHelper, IVisualTreeHelperStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualTreeHelperStatics3<R, F: FnOnce(&IVisualTreeHelperStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VisualTreeHelper, IVisualTreeHelperStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VisualTreeHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.VisualTreeHelper;{24b935e3-52c7-4141-8bac-a73d06130569})");
}
unsafe impl ::windows::core::Interface for VisualTreeHelper {
    type Vtable = IVisualTreeHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24b935e3_52c7_4141_8bac_a73d06130569);
}
impl ::windows::core::RuntimeName for VisualTreeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.VisualTreeHelper";
}
impl ::core::convert::From<VisualTreeHelper> for ::windows::core::IUnknown {
    fn from(value: VisualTreeHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VisualTreeHelper> for ::windows::core::IUnknown {
    fn from(value: &VisualTreeHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VisualTreeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VisualTreeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VisualTreeHelper> for ::windows::core::IInspectable {
    fn from(value: VisualTreeHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VisualTreeHelper> for ::windows::core::IInspectable {
    fn from(value: &VisualTreeHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VisualTreeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VisualTreeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VisualTreeHelper {}
unsafe impl ::core::marker::Sync for VisualTreeHelper {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlCompositionBrushBase(pub ::windows::core::IInspectable);
impl XamlCompositionBrushBase {
    pub fn FallbackColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetFallbackColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CompositionBrush(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush> {
        let this = &::windows::core::Interface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Composition::CompositionBrush>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetCompositionBrush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Composition::CompositionBrush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FallbackColorProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlCompositionBrushBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IXamlCompositionBrushBaseStatics<R, F: FnOnce(&IXamlCompositionBrushBaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlCompositionBrushBase, IXamlCompositionBrushBaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlCompositionBrushBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.XamlCompositionBrushBase;{03e432d9-b35c-4a79-811c-c5652004da0e})");
}
unsafe impl ::windows::core::Interface for XamlCompositionBrushBase {
    type Vtable = IXamlCompositionBrushBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e432d9_b35c_4a79_811c_c5652004da0e);
}
impl ::windows::core::RuntimeName for XamlCompositionBrushBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.XamlCompositionBrushBase";
}
impl ::core::convert::From<XamlCompositionBrushBase> for ::windows::core::IUnknown {
    fn from(value: XamlCompositionBrushBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlCompositionBrushBase> for ::windows::core::IUnknown {
    fn from(value: &XamlCompositionBrushBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlCompositionBrushBase> for ::windows::core::IInspectable {
    fn from(value: XamlCompositionBrushBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlCompositionBrushBase> for ::windows::core::IInspectable {
    fn from(value: &XamlCompositionBrushBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<XamlCompositionBrushBase> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: XamlCompositionBrushBase) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&XamlCompositionBrushBase> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &XamlCompositionBrushBase) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<XamlCompositionBrushBase> for Brush {
    fn from(value: XamlCompositionBrushBase) -> Self {
        ::core::convert::Into::<Brush>::into(&value)
    }
}
impl ::core::convert::From<&XamlCompositionBrushBase> for Brush {
    fn from(value: &XamlCompositionBrushBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Brush> for &XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, Brush> {
        ::windows::core::Param::Owned(::core::convert::Into::<Brush>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<XamlCompositionBrushBase> for super::DependencyObject {
    fn from(value: XamlCompositionBrushBase) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&XamlCompositionBrushBase> for super::DependencyObject {
    fn from(value: &XamlCompositionBrushBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &XamlCompositionBrushBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for XamlCompositionBrushBase {}
unsafe impl ::core::marker::Sync for XamlCompositionBrushBase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlLight(pub ::windows::core::IInspectable);
impl XamlLight {
    #[cfg(feature = "UI_Composition")]
    pub fn CompositionLight(&self) -> ::windows::core::Result<super::super::Composition::CompositionLight> {
        let this = &::windows::core::Interface::cast::<IXamlLightProtected>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Composition::CompositionLight>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetCompositionLight<'a, Param0: ::windows::core::IntoParam<'a, super::super::Composition::CompositionLight>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlLightProtected>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AddTargetElement<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(lightid: Param0, element: Param1) -> ::windows::core::Result<()> {
        Self::IXamlLightStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), lightid.into_param().abi(), element.into_param().abi()).ok() })
    }
    pub fn RemoveTargetElement<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(lightid: Param0, element: Param1) -> ::windows::core::Result<()> {
        Self::IXamlLightStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), lightid.into_param().abi(), element.into_param().abi()).ok() })
    }
    pub fn AddTargetBrush<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Brush>>(lightid: Param0, brush: Param1) -> ::windows::core::Result<()> {
        Self::IXamlLightStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), lightid.into_param().abi(), brush.into_param().abi()).ok() })
    }
    pub fn RemoveTargetBrush<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Brush>>(lightid: Param0, brush: Param1) -> ::windows::core::Result<()> {
        Self::IXamlLightStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), lightid.into_param().abi(), brush.into_param().abi()).ok() })
    }
    pub fn new() -> ::windows::core::Result<XamlLight> {
        Self::IXamlLightFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<XamlLight>(result__)
        })
    }
    pub fn IXamlLightStatics<R, F: FnOnce(&IXamlLightStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlLight, IXamlLightStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IXamlLightFactory<R, F: FnOnce(&IXamlLightFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlLight, IXamlLightFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlLight {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.XamlLight;{0cc3fc1f-b327-4a18-9648-7c84db26ce22})");
}
unsafe impl ::windows::core::Interface for XamlLight {
    type Vtable = IXamlLight_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cc3fc1f_b327_4a18_9648_7c84db26ce22);
}
impl ::windows::core::RuntimeName for XamlLight {
    const NAME: &'static str = "Windows.UI.Xaml.Media.XamlLight";
}
impl ::core::convert::From<XamlLight> for ::windows::core::IUnknown {
    fn from(value: XamlLight) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlLight> for ::windows::core::IUnknown {
    fn from(value: &XamlLight) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlLight {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlLight {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlLight> for ::windows::core::IInspectable {
    fn from(value: XamlLight) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlLight> for ::windows::core::IInspectable {
    fn from(value: &XamlLight) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlLight {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlLight {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<XamlLight> for super::DependencyObject {
    fn from(value: XamlLight) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&XamlLight> for super::DependencyObject {
    fn from(value: &XamlLight) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for XamlLight {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &XamlLight {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for XamlLight {}
unsafe impl ::core::marker::Sync for XamlLight {}
