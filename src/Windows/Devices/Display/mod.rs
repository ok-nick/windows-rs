#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayMonitor(pub ::windows::core::IInspectable);
impl DisplayMonitor {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ConnectionKind(&self) -> ::windows::core::Result<DisplayMonitorConnectionKind> {
        let this = self;
        unsafe {
            let mut result__: DisplayMonitorConnectionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayMonitorConnectionKind>(result__)
        }
    }
    pub fn PhysicalConnector(&self) -> ::windows::core::Result<DisplayMonitorPhysicalConnectorKind> {
        let this = self;
        unsafe {
            let mut result__: DisplayMonitorPhysicalConnectorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayMonitorPhysicalConnectorKind>(result__)
        }
    }
    pub fn DisplayAdapterDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn DisplayAdapterId(&self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::DisplayAdapterId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DisplayAdapterId>(result__)
        }
    }
    pub fn DisplayAdapterTargetId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn UsageKind(&self) -> ::windows::core::Result<DisplayMonitorUsageKind> {
        let this = self;
        unsafe {
            let mut result__: DisplayMonitorUsageKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayMonitorUsageKind>(result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn NativeResolutionInRawPixels(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::SizeInt32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PhysicalSizeInInches(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Size>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::Size>>(result__)
        }
    }
    pub fn RawDpiX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn RawDpiY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RedPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GreenPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BluePrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WhitePoint(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    pub fn MaxLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn MinLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn GetDescriptor(&self, descriptorkind: DisplayMonitorDescriptorKind) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), descriptorkind, ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DisplayMonitor>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromInterfaceIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceinterfaceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceinterfaceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DisplayMonitor>>(result__)
        })
    }
    pub fn IsDolbyVisionSupportedInHdrMode(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDisplayMonitor2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IDisplayMonitorStatics<R, F: FnOnce(&IDisplayMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayMonitor, IDisplayMonitorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.DisplayMonitor;{1f6b15d4-1d01-4c51-87e2-6f954a772b59})");
}
unsafe impl ::windows::core::Interface for DisplayMonitor {
    type Vtable = IDisplayMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f6b15d4_1d01_4c51_87e2_6f954a772b59);
}
impl ::windows::core::RuntimeName for DisplayMonitor {
    const NAME: &'static str = "Windows.Devices.Display.DisplayMonitor";
}
impl ::core::convert::From<DisplayMonitor> for ::windows::core::IUnknown {
    fn from(value: DisplayMonitor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayMonitor> for ::windows::core::IUnknown {
    fn from(value: &DisplayMonitor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayMonitor> for ::windows::core::IInspectable {
    fn from(value: DisplayMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayMonitor> for ::windows::core::IInspectable {
    fn from(value: &DisplayMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayMonitor {}
unsafe impl ::core::marker::Sync for DisplayMonitor {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayMonitorConnectionKind(pub i32);
impl DisplayMonitorConnectionKind {
    pub const Internal: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(0i32);
    pub const Wired: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(1i32);
    pub const Wireless: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(2i32);
    pub const Virtual: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(3i32);
}
impl ::core::convert::From<i32> for DisplayMonitorConnectionKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayMonitorConnectionKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayMonitorConnectionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorConnectionKind;i4)");
}
impl ::windows::core::DefaultType for DisplayMonitorConnectionKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayMonitorDescriptorKind(pub i32);
impl DisplayMonitorDescriptorKind {
    pub const Edid: DisplayMonitorDescriptorKind = DisplayMonitorDescriptorKind(0i32);
    pub const DisplayId: DisplayMonitorDescriptorKind = DisplayMonitorDescriptorKind(1i32);
}
impl ::core::convert::From<i32> for DisplayMonitorDescriptorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayMonitorDescriptorKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayMonitorDescriptorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorDescriptorKind;i4)");
}
impl ::windows::core::DefaultType for DisplayMonitorDescriptorKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayMonitorPhysicalConnectorKind(pub i32);
impl DisplayMonitorPhysicalConnectorKind {
    pub const Unknown: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(0i32);
    pub const HD15: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(1i32);
    pub const AnalogTV: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(2i32);
    pub const Dvi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(3i32);
    pub const Hdmi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(4i32);
    pub const Lvds: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(5i32);
    pub const Sdi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(6i32);
    pub const DisplayPort: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(7i32);
}
impl ::core::convert::From<i32> for DisplayMonitorPhysicalConnectorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayMonitorPhysicalConnectorKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayMonitorPhysicalConnectorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorPhysicalConnectorKind;i4)");
}
impl ::windows::core::DefaultType for DisplayMonitorPhysicalConnectorKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayMonitorUsageKind(pub i32);
impl DisplayMonitorUsageKind {
    pub const Standard: DisplayMonitorUsageKind = DisplayMonitorUsageKind(0i32);
    pub const HeadMounted: DisplayMonitorUsageKind = DisplayMonitorUsageKind(1i32);
    pub const SpecialPurpose: DisplayMonitorUsageKind = DisplayMonitorUsageKind(2i32);
}
impl ::core::convert::From<i32> for DisplayMonitorUsageKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayMonitorUsageKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayMonitorUsageKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorUsageKind;i4)");
}
impl ::windows::core::DefaultType for DisplayMonitorUsageKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayMonitor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayMonitor {
    type Vtable = IDisplayMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f6b15d4_1d01_4c51_87e2_6f954a772b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayMonitorConnectionKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayMonitorPhysicalConnectorKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayMonitorUsageKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descriptorkind: DisplayMonitorDescriptorKind, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayMonitor2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayMonitor2 {
    type Vtable = IDisplayMonitor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023018e6_cb23_5830_96df_a7bf6e602577);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayMonitorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayMonitorStatics {
    type Vtable = IDisplayMonitorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eae698f_a228_4c05_821d_b695d667de8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceinterfaceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
