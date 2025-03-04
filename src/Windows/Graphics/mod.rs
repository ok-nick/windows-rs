#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Graphics_Capture")]
pub mod Capture;
#[cfg(feature = "Graphics_DirectX")]
pub mod DirectX;
#[cfg(feature = "Graphics_Display")]
pub mod Display;
#[cfg(feature = "Graphics_Effects")]
pub mod Effects;
#[cfg(feature = "Graphics_Holographic")]
pub mod Holographic;
#[cfg(feature = "Graphics_Imaging")]
pub mod Imaging;
#[cfg(feature = "Graphics_Printing")]
pub mod Printing;
#[cfg(feature = "Graphics_Printing3D")]
pub mod Printing3D;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DisplayAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl DisplayAdapterId {}
impl ::core::default::Default for DisplayAdapterId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DisplayAdapterId {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DisplayAdapterId").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl ::core::cmp::PartialEq for DisplayAdapterId {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for DisplayAdapterId {}
unsafe impl ::windows::core::Abi for DisplayAdapterId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayAdapterId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.DisplayAdapterId;u4;i4)");
}
impl ::windows::core::DefaultType for DisplayAdapterId {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DisplayId {
    pub Value: u64,
}
impl DisplayId {}
impl ::core::default::Default for DisplayId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DisplayId {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DisplayId").field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for DisplayId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DisplayId {}
unsafe impl ::windows::core::Abi for DisplayId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.DisplayId;u8)");
}
impl ::windows::core::DefaultType for DisplayId {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGeometrySource2D(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeometrySource2D {
    type Vtable = IGeometrySource2D_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaff7902_670c_4181_a624_da977203b845);
}
impl IGeometrySource2D {}
unsafe impl ::windows::core::RuntimeType for IGeometrySource2D {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{caff7902-670c-4181-a624-da977203b845}");
}
impl ::core::convert::From<IGeometrySource2D> for ::windows::core::IUnknown {
    fn from(value: IGeometrySource2D) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGeometrySource2D> for ::windows::core::IUnknown {
    fn from(value: &IGeometrySource2D) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGeometrySource2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGeometrySource2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGeometrySource2D> for ::windows::core::IInspectable {
    fn from(value: IGeometrySource2D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGeometrySource2D> for ::windows::core::IInspectable {
    fn from(value: &IGeometrySource2D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGeometrySource2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGeometrySource2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2D_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PointInt32 {
    pub X: i32,
    pub Y: i32,
}
impl PointInt32 {}
impl ::core::default::Default for PointInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PointInt32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PointInt32").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::cmp::PartialEq for PointInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for PointInt32 {}
unsafe impl ::windows::core::Abi for PointInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PointInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.PointInt32;i4;i4)");
}
impl ::windows::core::DefaultType for PointInt32 {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct RectInt32 {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl RectInt32 {}
impl ::core::default::Default for RectInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RectInt32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RectInt32").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::cmp::PartialEq for RectInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for RectInt32 {}
unsafe impl ::windows::core::Abi for RectInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RectInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.RectInt32;i4;i4;i4;i4)");
}
impl ::windows::core::DefaultType for RectInt32 {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SizeInt32 {
    pub Width: i32,
    pub Height: i32,
}
impl SizeInt32 {}
impl ::core::default::Default for SizeInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SizeInt32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SizeInt32").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::cmp::PartialEq for SizeInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for SizeInt32 {}
unsafe impl ::windows::core::Abi for SizeInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SizeInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.SizeInt32;i4;i4)");
}
impl ::windows::core::DefaultType for SizeInt32 {
    type DefaultType = Self;
}
