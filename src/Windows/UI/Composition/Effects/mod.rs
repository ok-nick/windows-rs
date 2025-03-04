#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneLightingEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISceneLightingEffect {
    type Vtable = ISceneLightingEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91bb5e52_95d1_4f8b_9a5a_6408b24b8c6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneLightingEffect2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISceneLightingEffect2 {
    type Vtable = ISceneLightingEffect2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e270e81_72f0_4c5c_95f8_8a6e0024f409);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SceneLightingEffectReflectanceModel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: SceneLightingEffectReflectanceModel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneLightingEffect(pub ::windows::core::IInspectable);
impl SceneLightingEffect {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SceneLightingEffect, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Graphics_Effects")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Graphics_Effects")]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    pub fn AmbientAmount(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetAmbientAmount(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DiffuseAmount(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetDiffuseAmount(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Effects")]
    pub fn NormalMapSource(&self) -> ::windows::core::Result<super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Effects::IGraphicsEffectSource>(result__)
        }
    }
    #[cfg(feature = "Graphics_Effects")]
    pub fn SetNormalMapSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SpecularAmount(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetSpecularAmount(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SpecularShine(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetSpecularShine(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReflectanceModel(&self) -> ::windows::core::Result<SceneLightingEffectReflectanceModel> {
        let this = &::windows::core::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe {
            let mut result__: SceneLightingEffectReflectanceModel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneLightingEffectReflectanceModel>(result__)
        }
    }
    pub fn SetReflectanceModel(&self, value: SceneLightingEffectReflectanceModel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SceneLightingEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Effects.SceneLightingEffect;{91bb5e52-95d1-4f8b-9a5a-6408b24b8c6a})");
}
unsafe impl ::windows::core::Interface for SceneLightingEffect {
    type Vtable = ISceneLightingEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91bb5e52_95d1_4f8b_9a5a_6408b24b8c6a);
}
impl ::windows::core::RuntimeName for SceneLightingEffect {
    const NAME: &'static str = "Windows.UI.Composition.Effects.SceneLightingEffect";
}
impl ::core::convert::From<SceneLightingEffect> for ::windows::core::IUnknown {
    fn from(value: SceneLightingEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneLightingEffect> for ::windows::core::IUnknown {
    fn from(value: &SceneLightingEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneLightingEffect> for ::windows::core::IInspectable {
    fn from(value: SceneLightingEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneLightingEffect> for ::windows::core::IInspectable {
    fn from(value: &SceneLightingEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffect> for SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffect> for &SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffect> {
        ::core::convert::TryInto::<super::super::super::Graphics::Effects::IGraphicsEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> for SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> for &SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        ::core::convert::TryInto::<super::super::super::Graphics::Effects::IGraphicsEffectSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SceneLightingEffect {}
unsafe impl ::core::marker::Sync for SceneLightingEffect {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SceneLightingEffectReflectanceModel(pub i32);
impl SceneLightingEffectReflectanceModel {
    pub const BlinnPhong: SceneLightingEffectReflectanceModel = SceneLightingEffectReflectanceModel(0i32);
    pub const PhysicallyBasedBlinnPhong: SceneLightingEffectReflectanceModel = SceneLightingEffectReflectanceModel(1i32);
}
impl ::core::convert::From<i32> for SceneLightingEffectReflectanceModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SceneLightingEffectReflectanceModel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SceneLightingEffectReflectanceModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Effects.SceneLightingEffectReflectanceModel;i4)");
}
impl ::windows::core::DefaultType for SceneLightingEffectReflectanceModel {
    type DefaultType = Self;
}
