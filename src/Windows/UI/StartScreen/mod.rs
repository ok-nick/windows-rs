#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForegroundText(pub i32);
impl ForegroundText {
    pub const Dark: ForegroundText = ForegroundText(0i32);
    pub const Light: ForegroundText = ForegroundText(1i32);
}
impl ::core::convert::From<i32> for ForegroundText {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ForegroundText {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ForegroundText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.ForegroundText;i4)");
}
impl ::windows::core::DefaultType for ForegroundText {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IJumpList(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJumpList {
    type Vtable = IJumpList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0234c3e_cd6f_4cb6_a611_61fd505f3ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut JumpListSystemGroupKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: JumpListSystemGroupKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJumpListItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJumpListItem {
    type Vtable = IJumpListItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7adb6717_8b5d_4820_995b_9b418dbe48b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut JumpListItemKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJumpListItemStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJumpListItemStatics {
    type Vtable = IJumpListItemStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1bfc4e8_c7aa_49cb_8dde_ecfccd7ad7e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJumpListStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJumpListStatics {
    type Vtable = IJumpListStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7e0c681_e67e_4b74_8250_3f322c4d92c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTile(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTile {
    type Vtable = ISecondaryTile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e9e51e0_2bb5_4bc0_bb8d_42b23abcc88d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTile_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: TileOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TileOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ForegroundText) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ForegroundText) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTile2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTile2 {
    type Vtable = ISecondaryTile2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2f6cc35_3250_4990_923c_294ab4b694dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTile2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTileFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTileFactory {
    type Vtable = ISecondaryTileFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57f52ca0_51bc_4abf_8ebf_627a0398b05a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, shortname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tileoptions: TileOptions, logoreference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, shortname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tileoptions: TileOptions, logoreference: ::windows::core::RawPtr, widelogoreference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTileFactory2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTileFactory2 {
    type Vtable = ISecondaryTileFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x274b8a3b_522d_448e_9eb2_d0672ab345c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, square150x150logo: ::windows::core::RawPtr, desiredsize: TileSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTileStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTileStatics {
    type Vtable = ISecondaryTileStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99908dae_d051_4676_87fe_9ec242d83c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTileVisualElements {
    type Vtable = ISecondaryTileVisualElements_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d8df333_815e_413f_9f50_a81da70a96b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ForegroundText) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ForegroundText) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTileVisualElements2 {
    type Vtable = ISecondaryTileVisualElements2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd2e31d0_57dc_4794_8ecf_5682f5f3e6ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTileVisualElements3 {
    type Vtable = ISecondaryTileVisualElements3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56b55ad6_d15c_40f4_81e7_57ffd8f8a4e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecondaryTileVisualElements4 {
    type Vtable = ISecondaryTileVisualElements4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66566117_b544_40d2_8d12_74d4ec24d04c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements4_abi(
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
pub struct IStartScreenManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStartScreenManager {
    type Vtable = IStartScreenManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a1dcbcb_26e9_4eb4_8933_859eb6ecdb29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "ApplicationModel_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applistentry: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStartScreenManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStartScreenManager2 {
    type Vtable = IStartScreenManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08a716b6_316b_4ad9_acb8_fe9cf00bd608);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStartScreenManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStartScreenManagerStatics {
    type Vtable = IStartScreenManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7865ef0f_b585_464e_8993_34e8f8738d48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManagerStatics_abi(
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
pub struct ITileMixedRealityModel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileMixedRealityModel {
    type Vtable = ITileMixedRealityModel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0764e5b_887d_4242_9a19_3d0a4ea78031);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileMixedRealityModel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileMixedRealityModel2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileMixedRealityModel2 {
    type Vtable = ITileMixedRealityModel2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x439470b2_d7c5_410b_8319_9486a27b6c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileMixedRealityModel2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: TileMixedRealityModelActivationBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TileMixedRealityModelActivationBehavior) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualElementsRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualElementsRequest {
    type Vtable = IVisualElementsRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc138333a_9308_4072_88cc_d068db347c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualElementsRequestDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualElementsRequestDeferral {
    type Vtable = IVisualElementsRequestDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1656eb0_0126_4357_8204_bd82bb2a046d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequestDeferral_abi(
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
pub struct IVisualElementsRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualElementsRequestedEventArgs {
    type Vtable = IVisualElementsRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b6fc982_3a0d_4ece_af96_cd17e1b00b2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequestedEventArgs_abi(
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
pub struct JumpList(pub ::windows::core::IInspectable);
impl JumpList {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<JumpListItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<JumpListItem>>(result__)
        }
    }
    pub fn SystemGroupKind(&self) -> ::windows::core::Result<JumpListSystemGroupKind> {
        let this = self;
        unsafe {
            let mut result__: JumpListSystemGroupKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JumpListSystemGroupKind>(result__)
        }
    }
    pub fn SetSystemGroupKind(&self, value: JumpListSystemGroupKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoadCurrentAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<JumpList>> {
        Self::IJumpListStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<JumpList>>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IJumpListStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IJumpListStatics<R, F: FnOnce(&IJumpListStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JumpList, IJumpListStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for JumpList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.JumpList;{b0234c3e-cd6f-4cb6-a611-61fd505f3ed1})");
}
unsafe impl ::windows::core::Interface for JumpList {
    type Vtable = IJumpList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0234c3e_cd6f_4cb6_a611_61fd505f3ed1);
}
impl ::windows::core::RuntimeName for JumpList {
    const NAME: &'static str = "Windows.UI.StartScreen.JumpList";
}
impl ::core::convert::From<JumpList> for ::windows::core::IUnknown {
    fn from(value: JumpList) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&JumpList> for ::windows::core::IUnknown {
    fn from(value: &JumpList) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JumpList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JumpList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<JumpList> for ::windows::core::IInspectable {
    fn from(value: JumpList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&JumpList> for ::windows::core::IInspectable {
    fn from(value: &JumpList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JumpList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JumpList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for JumpList {}
unsafe impl ::core::marker::Sync for JumpList {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct JumpListItem(pub ::windows::core::IInspectable);
impl JumpListItem {
    pub fn Kind(&self) -> ::windows::core::Result<JumpListItemKind> {
        let this = self;
        unsafe {
            let mut result__: JumpListItemKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JumpListItemKind>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RemovedByUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn GroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetGroupName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLogo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CreateWithArguments<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(arguments: Param0, displayname: Param1) -> ::windows::core::Result<JumpListItem> {
        Self::IJumpListItemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), arguments.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<JumpListItem>(result__)
        })
    }
    pub fn CreateSeparator() -> ::windows::core::Result<JumpListItem> {
        Self::IJumpListItemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JumpListItem>(result__)
        })
    }
    pub fn IJumpListItemStatics<R, F: FnOnce(&IJumpListItemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JumpListItem, IJumpListItemStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for JumpListItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.JumpListItem;{7adb6717-8b5d-4820-995b-9b418dbe48b0})");
}
unsafe impl ::windows::core::Interface for JumpListItem {
    type Vtable = IJumpListItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7adb6717_8b5d_4820_995b_9b418dbe48b0);
}
impl ::windows::core::RuntimeName for JumpListItem {
    const NAME: &'static str = "Windows.UI.StartScreen.JumpListItem";
}
impl ::core::convert::From<JumpListItem> for ::windows::core::IUnknown {
    fn from(value: JumpListItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&JumpListItem> for ::windows::core::IUnknown {
    fn from(value: &JumpListItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JumpListItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JumpListItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<JumpListItem> for ::windows::core::IInspectable {
    fn from(value: JumpListItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&JumpListItem> for ::windows::core::IInspectable {
    fn from(value: &JumpListItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JumpListItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JumpListItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for JumpListItem {}
unsafe impl ::core::marker::Sync for JumpListItem {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JumpListItemKind(pub i32);
impl JumpListItemKind {
    pub const Arguments: JumpListItemKind = JumpListItemKind(0i32);
    pub const Separator: JumpListItemKind = JumpListItemKind(1i32);
}
impl ::core::convert::From<i32> for JumpListItemKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JumpListItemKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for JumpListItemKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.JumpListItemKind;i4)");
}
impl ::windows::core::DefaultType for JumpListItemKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JumpListSystemGroupKind(pub i32);
impl JumpListSystemGroupKind {
    pub const None: JumpListSystemGroupKind = JumpListSystemGroupKind(0i32);
    pub const Frequent: JumpListSystemGroupKind = JumpListSystemGroupKind(1i32);
    pub const Recent: JumpListSystemGroupKind = JumpListSystemGroupKind(2i32);
}
impl ::core::convert::From<i32> for JumpListSystemGroupKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JumpListSystemGroupKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for JumpListSystemGroupKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.JumpListSystemGroupKind;i4)");
}
impl ::windows::core::DefaultType for JumpListSystemGroupKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryTile(pub ::windows::core::IInspectable);
impl SecondaryTile {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryTile, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetTileId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetArguments<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetShortName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShortName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetLogo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetSmallLogo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SmallLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetWideLogo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn WideLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLockScreenBadgeLogo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LockScreenBadgeLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn SetLockScreenDisplayBadgeAndTileText(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LockScreenDisplayBadgeAndTileText(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetTileOptions(&self, value: TileOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn TileOptions(&self) -> ::windows::core::Result<TileOptions> {
        let this = self;
        unsafe {
            let mut result__: TileOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileOptions>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetForegroundText(&self, value: ForegroundText) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ForegroundText(&self) -> ::windows::core::Result<ForegroundText> {
        let this = self;
        unsafe {
            let mut result__: ForegroundText = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForegroundText>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsyncWithPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, invocationpoint: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), invocationpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsyncWithRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn RequestCreateAsyncWithRectAndPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsyncWithPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, invocationpoint: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), invocationpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsyncWithRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn RequestDeleteAsyncWithRectAndPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn SetPhoneticName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PhoneticName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn VisualElements(&self) -> ::windows::core::Result<SecondaryTileVisualElements> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryTileVisualElements>(result__)
        }
    }
    pub fn SetRoamingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RoamingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn VisualElementsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SecondaryTile, VisualElementsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisualElementsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn CreateTile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(
        tileid: Param0,
        shortname: Param1,
        displayname: Param2,
        arguments: Param3,
        tileoptions: TileOptions,
        logoreference: Param5,
    ) -> ::windows::core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tileid.into_param().abi(), shortname.into_param().abi(), displayname.into_param().abi(), arguments.into_param().abi(), tileoptions, logoreference.into_param().abi(), &mut result__).from_abi::<SecondaryTile>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn CreateWideTile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(
        tileid: Param0,
        shortname: Param1,
        displayname: Param2,
        arguments: Param3,
        tileoptions: TileOptions,
        logoreference: Param5,
        widelogoreference: Param6,
    ) -> ::windows::core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), tileid.into_param().abi(), shortname.into_param().abi(), displayname.into_param().abi(), arguments.into_param().abi(), tileoptions, logoreference.into_param().abi(), widelogoreference.into_param().abi(), &mut result__).from_abi::<SecondaryTile>(result__)
        })
    }
    pub fn CreateWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tileid: Param0) -> ::windows::core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<SecondaryTile>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateMinimalTile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(tileid: Param0, displayname: Param1, arguments: Param2, square150x150logo: Param3, desiredsize: TileSize) -> ::windows::core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tileid.into_param().abi(), displayname.into_param().abi(), arguments.into_param().abi(), square150x150logo.into_param().abi(), desiredsize, &mut result__).from_abi::<SecondaryTile>(result__)
        })
    }
    pub fn Exists<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tileid: Param0) -> ::windows::core::Result<bool> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllForApplicationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllForPackageAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>(result__)
        })
    }
    pub fn ISecondaryTileFactory<R, F: FnOnce(&ISecondaryTileFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryTile, ISecondaryTileFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISecondaryTileFactory2<R, F: FnOnce(&ISecondaryTileFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryTile, ISecondaryTileFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISecondaryTileStatics<R, F: FnOnce(&ISecondaryTileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryTile, ISecondaryTileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SecondaryTile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.SecondaryTile;{9e9e51e0-2bb5-4bc0-bb8d-42b23abcc88d})");
}
unsafe impl ::windows::core::Interface for SecondaryTile {
    type Vtable = ISecondaryTile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e9e51e0_2bb5_4bc0_bb8d_42b23abcc88d);
}
impl ::windows::core::RuntimeName for SecondaryTile {
    const NAME: &'static str = "Windows.UI.StartScreen.SecondaryTile";
}
impl ::core::convert::From<SecondaryTile> for ::windows::core::IUnknown {
    fn from(value: SecondaryTile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryTile> for ::windows::core::IUnknown {
    fn from(value: &SecondaryTile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryTile> for ::windows::core::IInspectable {
    fn from(value: SecondaryTile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryTile> for ::windows::core::IInspectable {
    fn from(value: &SecondaryTile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryTile {}
unsafe impl ::core::marker::Sync for SecondaryTile {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryTileVisualElements(pub ::windows::core::IInspectable);
impl SecondaryTileVisualElements {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare30x30Logo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare70x70Logo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Square70x70Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSquare150x150Logo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Square150x150Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetWide310x150Logo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Wide310x150Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSquare310x310Logo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Square310x310Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn SetForegroundText(&self, value: ForegroundText) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForegroundText(&self) -> ::windows::core::Result<ForegroundText> {
        let this = self;
        unsafe {
            let mut result__: ForegroundText = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForegroundText>(result__)
        }
    }
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        }
    }
    pub fn SetShowNameOnSquare150x150Logo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ShowNameOnSquare150x150Logo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShowNameOnWide310x150Logo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ShowNameOnWide310x150Logo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShowNameOnSquare310x310Logo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ShowNameOnSquare310x310Logo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSquare71x71Logo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Square71x71Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSquare44x44Logo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Square44x44Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn MixedRealityModel(&self) -> ::windows::core::Result<TileMixedRealityModel> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileMixedRealityModel>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SecondaryTileVisualElements {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.SecondaryTileVisualElements;{1d8df333-815e-413f-9f50-a81da70a96b2})");
}
unsafe impl ::windows::core::Interface for SecondaryTileVisualElements {
    type Vtable = ISecondaryTileVisualElements_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d8df333_815e_413f_9f50_a81da70a96b2);
}
impl ::windows::core::RuntimeName for SecondaryTileVisualElements {
    const NAME: &'static str = "Windows.UI.StartScreen.SecondaryTileVisualElements";
}
impl ::core::convert::From<SecondaryTileVisualElements> for ::windows::core::IUnknown {
    fn from(value: SecondaryTileVisualElements) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryTileVisualElements> for ::windows::core::IUnknown {
    fn from(value: &SecondaryTileVisualElements) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryTileVisualElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryTileVisualElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryTileVisualElements> for ::windows::core::IInspectable {
    fn from(value: SecondaryTileVisualElements) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryTileVisualElements> for ::windows::core::IInspectable {
    fn from(value: &SecondaryTileVisualElements) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryTileVisualElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryTileVisualElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryTileVisualElements {}
unsafe impl ::core::marker::Sync for SecondaryTileVisualElements {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StartScreenManager(pub ::windows::core::IInspectable);
impl StartScreenManager {
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn SupportsAppListEntry<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn ContainsAppListEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestAddAppListEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<StartScreenManager> {
        Self::IStartScreenManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StartScreenManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<StartScreenManager> {
        Self::IStartScreenManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<StartScreenManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ContainsSecondaryTileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IStartScreenManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryRemoveSecondaryTileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IStartScreenManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn IStartScreenManagerStatics<R, F: FnOnce(&IStartScreenManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StartScreenManager, IStartScreenManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StartScreenManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.StartScreenManager;{4a1dcbcb-26e9-4eb4-8933-859eb6ecdb29})");
}
unsafe impl ::windows::core::Interface for StartScreenManager {
    type Vtable = IStartScreenManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a1dcbcb_26e9_4eb4_8933_859eb6ecdb29);
}
impl ::windows::core::RuntimeName for StartScreenManager {
    const NAME: &'static str = "Windows.UI.StartScreen.StartScreenManager";
}
impl ::core::convert::From<StartScreenManager> for ::windows::core::IUnknown {
    fn from(value: StartScreenManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StartScreenManager> for ::windows::core::IUnknown {
    fn from(value: &StartScreenManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StartScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StartScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StartScreenManager> for ::windows::core::IInspectable {
    fn from(value: StartScreenManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StartScreenManager> for ::windows::core::IInspectable {
    fn from(value: &StartScreenManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StartScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StartScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StartScreenManager {}
unsafe impl ::core::marker::Sync for StartScreenManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileMixedRealityModel(pub ::windows::core::IInspectable);
impl TileMixedRealityModel {
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetBoundingBox<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>(result__)
        }
    }
    pub fn SetActivationBehavior(&self, value: TileMixedRealityModelActivationBehavior) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITileMixedRealityModel2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ActivationBehavior(&self) -> ::windows::core::Result<TileMixedRealityModelActivationBehavior> {
        let this = &::windows::core::Interface::cast::<ITileMixedRealityModel2>(self)?;
        unsafe {
            let mut result__: TileMixedRealityModelActivationBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileMixedRealityModelActivationBehavior>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TileMixedRealityModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.TileMixedRealityModel;{b0764e5b-887d-4242-9a19-3d0a4ea78031})");
}
unsafe impl ::windows::core::Interface for TileMixedRealityModel {
    type Vtable = ITileMixedRealityModel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0764e5b_887d_4242_9a19_3d0a4ea78031);
}
impl ::windows::core::RuntimeName for TileMixedRealityModel {
    const NAME: &'static str = "Windows.UI.StartScreen.TileMixedRealityModel";
}
impl ::core::convert::From<TileMixedRealityModel> for ::windows::core::IUnknown {
    fn from(value: TileMixedRealityModel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileMixedRealityModel> for ::windows::core::IUnknown {
    fn from(value: &TileMixedRealityModel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileMixedRealityModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TileMixedRealityModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileMixedRealityModel> for ::windows::core::IInspectable {
    fn from(value: TileMixedRealityModel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileMixedRealityModel> for ::windows::core::IInspectable {
    fn from(value: &TileMixedRealityModel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileMixedRealityModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TileMixedRealityModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TileMixedRealityModel {}
unsafe impl ::core::marker::Sync for TileMixedRealityModel {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TileMixedRealityModelActivationBehavior(pub i32);
impl TileMixedRealityModelActivationBehavior {
    pub const Default: TileMixedRealityModelActivationBehavior = TileMixedRealityModelActivationBehavior(0i32);
    pub const None: TileMixedRealityModelActivationBehavior = TileMixedRealityModelActivationBehavior(1i32);
}
impl ::core::convert::From<i32> for TileMixedRealityModelActivationBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TileMixedRealityModelActivationBehavior {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TileMixedRealityModelActivationBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileMixedRealityModelActivationBehavior;i4)");
}
impl ::windows::core::DefaultType for TileMixedRealityModelActivationBehavior {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TileOptions(pub u32);
impl TileOptions {
    pub const None: TileOptions = TileOptions(0u32);
    pub const ShowNameOnLogo: TileOptions = TileOptions(1u32);
    pub const ShowNameOnWideLogo: TileOptions = TileOptions(2u32);
    pub const CopyOnDeployment: TileOptions = TileOptions(4u32);
}
impl ::core::convert::From<u32> for TileOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TileOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TileOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileOptions;u4)");
}
impl ::windows::core::DefaultType for TileOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for TileOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TileOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TileOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TileOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TileSize(pub i32);
impl TileSize {
    pub const Default: TileSize = TileSize(0i32);
    pub const Square30x30: TileSize = TileSize(1i32);
    pub const Square70x70: TileSize = TileSize(2i32);
    pub const Square150x150: TileSize = TileSize(3i32);
    pub const Wide310x150: TileSize = TileSize(4i32);
    pub const Square310x310: TileSize = TileSize(5i32);
    pub const Square71x71: TileSize = TileSize(6i32);
    pub const Square44x44: TileSize = TileSize(7i32);
}
impl ::core::convert::From<i32> for TileSize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TileSize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TileSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileSize;i4)");
}
impl ::windows::core::DefaultType for TileSize {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VisualElementsRequest(pub ::windows::core::IInspectable);
impl VisualElementsRequest {
    pub fn VisualElements(&self) -> ::windows::core::Result<SecondaryTileVisualElements> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryTileVisualElements>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateVisualElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SecondaryTileVisualElements>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SecondaryTileVisualElements>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<VisualElementsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VisualElementsRequestDeferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VisualElementsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequest;{c138333a-9308-4072-88cc-d068db347c68})");
}
unsafe impl ::windows::core::Interface for VisualElementsRequest {
    type Vtable = IVisualElementsRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc138333a_9308_4072_88cc_d068db347c68);
}
impl ::windows::core::RuntimeName for VisualElementsRequest {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequest";
}
impl ::core::convert::From<VisualElementsRequest> for ::windows::core::IUnknown {
    fn from(value: VisualElementsRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VisualElementsRequest> for ::windows::core::IUnknown {
    fn from(value: &VisualElementsRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VisualElementsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VisualElementsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VisualElementsRequest> for ::windows::core::IInspectable {
    fn from(value: VisualElementsRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VisualElementsRequest> for ::windows::core::IInspectable {
    fn from(value: &VisualElementsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VisualElementsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VisualElementsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VisualElementsRequest {}
unsafe impl ::core::marker::Sync for VisualElementsRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VisualElementsRequestDeferral(pub ::windows::core::IInspectable);
impl VisualElementsRequestDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VisualElementsRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequestDeferral;{a1656eb0-0126-4357-8204-bd82bb2a046d})");
}
unsafe impl ::windows::core::Interface for VisualElementsRequestDeferral {
    type Vtable = IVisualElementsRequestDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1656eb0_0126_4357_8204_bd82bb2a046d);
}
impl ::windows::core::RuntimeName for VisualElementsRequestDeferral {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequestDeferral";
}
impl ::core::convert::From<VisualElementsRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: VisualElementsRequestDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VisualElementsRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &VisualElementsRequestDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VisualElementsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VisualElementsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VisualElementsRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: VisualElementsRequestDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VisualElementsRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &VisualElementsRequestDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VisualElementsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VisualElementsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VisualElementsRequestDeferral {}
unsafe impl ::core::marker::Sync for VisualElementsRequestDeferral {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VisualElementsRequestedEventArgs(pub ::windows::core::IInspectable);
impl VisualElementsRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<VisualElementsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VisualElementsRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VisualElementsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequestedEventArgs;{7b6fc982-3a0d-4ece-af96-cd17e1b00b2d})");
}
unsafe impl ::windows::core::Interface for VisualElementsRequestedEventArgs {
    type Vtable = IVisualElementsRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b6fc982_3a0d_4ece_af96_cd17e1b00b2d);
}
impl ::windows::core::RuntimeName for VisualElementsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequestedEventArgs";
}
impl ::core::convert::From<VisualElementsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VisualElementsRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VisualElementsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VisualElementsRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VisualElementsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VisualElementsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VisualElementsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VisualElementsRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VisualElementsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VisualElementsRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VisualElementsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VisualElementsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VisualElementsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for VisualElementsRequestedEventArgs {}
