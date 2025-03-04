#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct GpioChangeCount {
    pub Count: u64,
    pub RelativeTime: super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl GpioChangeCount {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for GpioChangeCount {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for GpioChangeCount {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GpioChangeCount").field("Count", &self.Count).field("RelativeTime", &self.RelativeTime).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for GpioChangeCount {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.RelativeTime == other.RelativeTime
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for GpioChangeCount {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for GpioChangeCount {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for GpioChangeCount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Gpio.GpioChangeCount;u8;struct(Windows.Foundation.TimeSpan;i8))");
}
#[cfg(feature = "Foundation")]
impl ::windows::core::DefaultType for GpioChangeCount {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GpioChangeCounter(pub ::windows::core::IInspectable);
impl GpioChangeCounter {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity> {
        let this = self;
        unsafe {
            let mut result__: GpioChangePolarity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangePolarity>(result__)
        }
    }
    pub fn IsStarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Read(&self) -> ::windows::core::Result<GpioChangeCount> {
        let this = self;
        unsafe {
            let mut result__: GpioChangeCount = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangeCount>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Reset(&self) -> ::windows::core::Result<GpioChangeCount> {
        let this = self;
        unsafe {
            let mut result__: GpioChangeCount = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangeCount>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, GpioPin>>(pin: Param0) -> ::windows::core::Result<GpioChangeCounter> {
        Self::IGpioChangeCounterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pin.into_param().abi(), &mut result__).from_abi::<GpioChangeCounter>(result__)
        })
    }
    pub fn IGpioChangeCounterFactory<R, F: FnOnce(&IGpioChangeCounterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioChangeCounter, IGpioChangeCounterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GpioChangeCounter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioChangeCounter;{cb5ec0de-6801-43ff-803d-4576628a8b26})");
}
unsafe impl ::windows::core::Interface for GpioChangeCounter {
    type Vtable = IGpioChangeCounter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb5ec0de_6801_43ff_803d_4576628a8b26);
}
impl ::windows::core::RuntimeName for GpioChangeCounter {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioChangeCounter";
}
impl ::core::convert::From<GpioChangeCounter> for ::windows::core::IUnknown {
    fn from(value: GpioChangeCounter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GpioChangeCounter> for ::windows::core::IUnknown {
    fn from(value: &GpioChangeCounter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GpioChangeCounter> for ::windows::core::IInspectable {
    fn from(value: GpioChangeCounter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GpioChangeCounter> for ::windows::core::IInspectable {
    fn from(value: &GpioChangeCounter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioChangeCounter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioChangeCounter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioChangeCounter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioChangeCounter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GpioChangeCounter {}
unsafe impl ::core::marker::Sync for GpioChangeCounter {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GpioChangePolarity(pub i32);
impl GpioChangePolarity {
    pub const Falling: GpioChangePolarity = GpioChangePolarity(0i32);
    pub const Rising: GpioChangePolarity = GpioChangePolarity(1i32);
    pub const Both: GpioChangePolarity = GpioChangePolarity(2i32);
}
impl ::core::convert::From<i32> for GpioChangePolarity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GpioChangePolarity {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GpioChangePolarity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioChangePolarity;i4)");
}
impl ::windows::core::DefaultType for GpioChangePolarity {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GpioChangeReader(pub ::windows::core::IInspectable);
impl GpioChangeReader {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Capacity(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn IsEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsOverflowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity> {
        let this = self;
        unsafe {
            let mut result__: GpioChangePolarity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangePolarity>(result__)
        }
    }
    pub fn IsStarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetNextItem(&self) -> ::windows::core::Result<GpioChangeRecord> {
        let this = self;
        unsafe {
            let mut result__: GpioChangeRecord = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangeRecord>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PeekNextItem(&self) -> ::windows::core::Result<GpioChangeRecord> {
        let this = self;
        unsafe {
            let mut result__: GpioChangeRecord = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangeRecord>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetAllItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<GpioChangeRecord>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<GpioChangeRecord>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WaitForItemsAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, GpioPin>>(pin: Param0) -> ::windows::core::Result<GpioChangeReader> {
        Self::IGpioChangeReaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pin.into_param().abi(), &mut result__).from_abi::<GpioChangeReader>(result__)
        })
    }
    pub fn CreateWithCapacity<'a, Param0: ::windows::core::IntoParam<'a, GpioPin>>(pin: Param0, mincapacity: i32) -> ::windows::core::Result<GpioChangeReader> {
        Self::IGpioChangeReaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pin.into_param().abi(), mincapacity, &mut result__).from_abi::<GpioChangeReader>(result__)
        })
    }
    pub fn IGpioChangeReaderFactory<R, F: FnOnce(&IGpioChangeReaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioChangeReader, IGpioChangeReaderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GpioChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioChangeReader;{0abc885f-e031-48e8-8590-70de78363c6d})");
}
unsafe impl ::windows::core::Interface for GpioChangeReader {
    type Vtable = IGpioChangeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0abc885f_e031_48e8_8590_70de78363c6d);
}
impl ::windows::core::RuntimeName for GpioChangeReader {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioChangeReader";
}
impl ::core::convert::From<GpioChangeReader> for ::windows::core::IUnknown {
    fn from(value: GpioChangeReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GpioChangeReader> for ::windows::core::IUnknown {
    fn from(value: &GpioChangeReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GpioChangeReader> for ::windows::core::IInspectable {
    fn from(value: GpioChangeReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GpioChangeReader> for ::windows::core::IInspectable {
    fn from(value: &GpioChangeReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioChangeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioChangeReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioChangeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioChangeReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GpioChangeReader {}
unsafe impl ::core::marker::Sync for GpioChangeReader {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct GpioChangeRecord {
    pub RelativeTime: super::super::Foundation::TimeSpan,
    pub Edge: GpioPinEdge,
}
#[cfg(feature = "Foundation")]
impl GpioChangeRecord {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for GpioChangeRecord {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for GpioChangeRecord {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GpioChangeRecord").field("RelativeTime", &self.RelativeTime).field("Edge", &self.Edge).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for GpioChangeRecord {
    fn eq(&self, other: &Self) -> bool {
        self.RelativeTime == other.RelativeTime && self.Edge == other.Edge
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for GpioChangeRecord {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for GpioChangeRecord {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for GpioChangeRecord {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Gpio.GpioChangeRecord;struct(Windows.Foundation.TimeSpan;i8);enum(Windows.Devices.Gpio.GpioPinEdge;i4))");
}
#[cfg(feature = "Foundation")]
impl ::windows::core::DefaultType for GpioChangeRecord {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GpioController(pub ::windows::core::IInspectable);
impl GpioController {
    pub fn PinCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<GpioPin> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pinnumber, &mut result__).from_abi::<GpioPin>(result__)
        }
    }
    pub fn OpenPinWithSharingMode(&self, pinnumber: i32, sharingmode: GpioSharingMode) -> ::windows::core::Result<GpioPin> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pinnumber, sharingmode, &mut result__).from_abi::<GpioPin>(result__)
        }
    }
    pub fn TryOpenPin(&self, pinnumber: i32, sharingmode: GpioSharingMode, pin: &mut ::core::option::Option<GpioPin>, openstatus: &mut GpioOpenStatus) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), pinnumber, sharingmode, pin as *mut _ as _, openstatus, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<GpioController> {
        Self::IGpioControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioController>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<'a, Param0: ::windows::core::IntoParam<'a, Provider::IGpioProvider>>(provider: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<GpioController>>> {
        Self::IGpioControllerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<GpioController>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GpioController>> {
        Self::IGpioControllerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GpioController>>(result__)
        })
    }
    pub fn IGpioControllerStatics<R, F: FnOnce(&IGpioControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioController, IGpioControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGpioControllerStatics2<R, F: FnOnce(&IGpioControllerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioController, IGpioControllerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GpioController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioController;{284012e3-7461-469c-a8bc-61d69d08a53c})");
}
unsafe impl ::windows::core::Interface for GpioController {
    type Vtable = IGpioController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x284012e3_7461_469c_a8bc_61d69d08a53c);
}
impl ::windows::core::RuntimeName for GpioController {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioController";
}
impl ::core::convert::From<GpioController> for ::windows::core::IUnknown {
    fn from(value: GpioController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GpioController> for ::windows::core::IUnknown {
    fn from(value: &GpioController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GpioController> for ::windows::core::IInspectable {
    fn from(value: GpioController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GpioController> for ::windows::core::IInspectable {
    fn from(value: &GpioController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GpioController {}
unsafe impl ::core::marker::Sync for GpioController {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GpioOpenStatus(pub i32);
impl GpioOpenStatus {
    pub const PinOpened: GpioOpenStatus = GpioOpenStatus(0i32);
    pub const PinUnavailable: GpioOpenStatus = GpioOpenStatus(1i32);
    pub const SharingViolation: GpioOpenStatus = GpioOpenStatus(2i32);
    pub const MuxingConflict: GpioOpenStatus = GpioOpenStatus(3i32);
    pub const UnknownError: GpioOpenStatus = GpioOpenStatus(4i32);
}
impl ::core::convert::From<i32> for GpioOpenStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GpioOpenStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GpioOpenStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioOpenStatus;i4)");
}
impl ::windows::core::DefaultType for GpioOpenStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GpioPin(pub ::windows::core::IInspectable);
impl GpioPin {
    #[cfg(feature = "Foundation")]
    pub fn ValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GpioPin, GpioPinValueChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DebounceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDebounceTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PinNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<GpioSharingMode> {
        let this = self;
        unsafe {
            let mut result__: GpioSharingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioSharingMode>(result__)
        }
    }
    pub fn IsDriveModeSupported(&self, drivemode: GpioPinDriveMode) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), drivemode, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetDriveMode(&self) -> ::windows::core::Result<GpioPinDriveMode> {
        let this = self;
        unsafe {
            let mut result__: GpioPinDriveMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioPinDriveMode>(result__)
        }
    }
    pub fn SetDriveMode(&self, value: GpioPinDriveMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Write(&self, value: GpioPinValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Read(&self) -> ::windows::core::Result<GpioPinValue> {
        let this = self;
        unsafe {
            let mut result__: GpioPinValue = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioPinValue>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioPin;{11d9b087-afae-4790-9ee9-e0eac942d201})");
}
unsafe impl ::windows::core::Interface for GpioPin {
    type Vtable = IGpioPin_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d9b087_afae_4790_9ee9_e0eac942d201);
}
impl ::windows::core::RuntimeName for GpioPin {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioPin";
}
impl ::core::convert::From<GpioPin> for ::windows::core::IUnknown {
    fn from(value: GpioPin) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GpioPin> for ::windows::core::IUnknown {
    fn from(value: &GpioPin) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GpioPin> for ::windows::core::IInspectable {
    fn from(value: GpioPin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GpioPin> for ::windows::core::IInspectable {
    fn from(value: &GpioPin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioPin) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioPin) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GpioPin {}
unsafe impl ::core::marker::Sync for GpioPin {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GpioPinDriveMode(pub i32);
impl GpioPinDriveMode {
    pub const Input: GpioPinDriveMode = GpioPinDriveMode(0i32);
    pub const Output: GpioPinDriveMode = GpioPinDriveMode(1i32);
    pub const InputPullUp: GpioPinDriveMode = GpioPinDriveMode(2i32);
    pub const InputPullDown: GpioPinDriveMode = GpioPinDriveMode(3i32);
    pub const OutputOpenDrain: GpioPinDriveMode = GpioPinDriveMode(4i32);
    pub const OutputOpenDrainPullUp: GpioPinDriveMode = GpioPinDriveMode(5i32);
    pub const OutputOpenSource: GpioPinDriveMode = GpioPinDriveMode(6i32);
    pub const OutputOpenSourcePullDown: GpioPinDriveMode = GpioPinDriveMode(7i32);
}
impl ::core::convert::From<i32> for GpioPinDriveMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GpioPinDriveMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GpioPinDriveMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinDriveMode;i4)");
}
impl ::windows::core::DefaultType for GpioPinDriveMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GpioPinEdge(pub i32);
impl GpioPinEdge {
    pub const FallingEdge: GpioPinEdge = GpioPinEdge(0i32);
    pub const RisingEdge: GpioPinEdge = GpioPinEdge(1i32);
}
impl ::core::convert::From<i32> for GpioPinEdge {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GpioPinEdge {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GpioPinEdge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinEdge;i4)");
}
impl ::windows::core::DefaultType for GpioPinEdge {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GpioPinValue(pub i32);
impl GpioPinValue {
    pub const Low: GpioPinValue = GpioPinValue(0i32);
    pub const High: GpioPinValue = GpioPinValue(1i32);
}
impl ::core::convert::From<i32> for GpioPinValue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GpioPinValue {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GpioPinValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinValue;i4)");
}
impl ::windows::core::DefaultType for GpioPinValue {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GpioPinValueChangedEventArgs(pub ::windows::core::IInspectable);
impl GpioPinValueChangedEventArgs {
    pub fn Edge(&self) -> ::windows::core::Result<GpioPinEdge> {
        let this = self;
        unsafe {
            let mut result__: GpioPinEdge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioPinEdge>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinValueChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioPinValueChangedEventArgs;{3137aae1-703d-4059-bd24-b5b25dffb84e})");
}
unsafe impl ::windows::core::Interface for GpioPinValueChangedEventArgs {
    type Vtable = IGpioPinValueChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3137aae1_703d_4059_bd24_b5b25dffb84e);
}
impl ::windows::core::RuntimeName for GpioPinValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioPinValueChangedEventArgs";
}
impl ::core::convert::From<GpioPinValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GpioPinValueChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GpioPinValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GpioPinValueChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioPinValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioPinValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GpioPinValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GpioPinValueChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GpioPinValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GpioPinValueChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioPinValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioPinValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GpioPinValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for GpioPinValueChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GpioSharingMode(pub i32);
impl GpioSharingMode {
    pub const Exclusive: GpioSharingMode = GpioSharingMode(0i32);
    pub const SharedReadOnly: GpioSharingMode = GpioSharingMode(1i32);
}
impl ::core::convert::From<i32> for GpioSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GpioSharingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GpioSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioSharingMode;i4)");
}
impl ::windows::core::DefaultType for GpioSharingMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioChangeCounter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioChangeCounter {
    type Vtable = IGpioChangeCounter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb5ec0de_6801_43ff_803d_4576628a8b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeCounter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GpioChangePolarity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioChangePolarity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioChangeCount) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioChangeCount) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioChangeCounterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioChangeCounterFactory {
    type Vtable = IGpioChangeCounterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x147d94b6_0a9e_410c_b4fa_f89f4052084d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeCounterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioChangeReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioChangeReader {
    type Vtable = IGpioChangeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0abc885f_e031_48e8_8590_70de78363c6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GpioChangePolarity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioChangePolarity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioChangeRecord) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioChangeRecord) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioChangeReaderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioChangeReaderFactory {
    type Vtable = IGpioChangeReaderFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9598ef3_390e_441a_9d1c_e8de0b2df0df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeReaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::windows::core::RawPtr, mincapacity: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioController {
    type Vtable = IGpioController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x284012e3_7461_469c_a8bc_61d69d08a53c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinnumber: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinnumber: i32, sharingmode: GpioSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinnumber: i32, sharingmode: GpioSharingMode, pin: *mut ::windows::core::RawPtr, openstatus: *mut GpioOpenStatus, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioControllerStatics {
    type Vtable = IGpioControllerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ed6f42e_7af7_4116_9533_c43d99a1fb64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerStatics_abi(
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
pub struct IGpioControllerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioControllerStatics2 {
    type Vtable = IGpioControllerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x912b7d20_6ca4_4106_a373_fffd346b0e5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Gpio_Provider", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioPin(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioPin {
    type Vtable = IGpioPin_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d9b087_afae_4790_9ee9_e0eac942d201);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPin_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioSharingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, drivemode: GpioPinDriveMode, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioPinDriveMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GpioPinDriveMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GpioPinValue) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioPinValue) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioPinValueChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioPinValueChangedEventArgs {
    type Vtable = IGpioPinValueChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3137aae1_703d_4059_bd24_b5b25dffb84e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinValueChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GpioPinEdge) -> ::windows::core::HRESULT,
);
