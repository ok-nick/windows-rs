#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WEB_SOCKET_ACTION(pub i32);
pub const WEB_SOCKET_NO_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(0i32);
pub const WEB_SOCKET_SEND_TO_NETWORK_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(1i32);
pub const WEB_SOCKET_INDICATE_SEND_COMPLETE_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(2i32);
pub const WEB_SOCKET_RECEIVE_FROM_NETWORK_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(3i32);
pub const WEB_SOCKET_INDICATE_RECEIVE_COMPLETE_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(4i32);
impl ::core::convert::From<i32> for WEB_SOCKET_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_ACTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WEB_SOCKET_ACTION_QUEUE(pub i32);
pub const WEB_SOCKET_SEND_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(1i32);
pub const WEB_SOCKET_RECEIVE_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(2i32);
pub const WEB_SOCKET_ALL_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(3i32);
impl ::core::convert::From<i32> for WEB_SOCKET_ACTION_QUEUE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_ACTION_QUEUE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union WEB_SOCKET_BUFFER {
    pub Data: WEB_SOCKET_BUFFER_1,
    pub CloseStatus: WEB_SOCKET_BUFFER_0,
}
impl WEB_SOCKET_BUFFER {}
impl ::core::default::Default for WEB_SOCKET_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEB_SOCKET_BUFFER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_BUFFER {}
unsafe impl ::windows::core::Abi for WEB_SOCKET_BUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WEB_SOCKET_BUFFER_0 {
    pub pbReason: *mut u8,
    pub ulReasonLength: u32,
    pub usStatus: u16,
}
impl WEB_SOCKET_BUFFER_0 {}
impl ::core::default::Default for WEB_SOCKET_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_BUFFER_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_CloseStatus_e__Struct").field("pbReason", &self.pbReason).field("ulReasonLength", &self.ulReasonLength).field("usStatus", &self.usStatus).finish()
    }
}
impl ::core::cmp::PartialEq for WEB_SOCKET_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pbReason == other.pbReason && self.ulReasonLength == other.ulReasonLength && self.usStatus == other.usStatus
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_BUFFER_0 {}
unsafe impl ::windows::core::Abi for WEB_SOCKET_BUFFER_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WEB_SOCKET_BUFFER_1 {
    pub pbBuffer: *mut u8,
    pub ulBufferLength: u32,
}
impl WEB_SOCKET_BUFFER_1 {}
impl ::core::default::Default for WEB_SOCKET_BUFFER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_BUFFER_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Data_e__Struct").field("pbBuffer", &self.pbBuffer).field("ulBufferLength", &self.ulBufferLength).finish()
    }
}
impl ::core::cmp::PartialEq for WEB_SOCKET_BUFFER_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pbBuffer == other.pbBuffer && self.ulBufferLength == other.ulBufferLength
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_BUFFER_1 {}
unsafe impl ::windows::core::Abi for WEB_SOCKET_BUFFER_1 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WEB_SOCKET_BUFFER_TYPE(pub i32);
pub const WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483648i32);
pub const WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483647i32);
pub const WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483646i32);
pub const WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483645i32);
pub const WEB_SOCKET_CLOSE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483644i32);
pub const WEB_SOCKET_PING_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483643i32);
pub const WEB_SOCKET_UNSOLICITED_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483642i32);
impl ::core::convert::From<i32> for WEB_SOCKET_BUFFER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_BUFFER_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WEB_SOCKET_CLOSE_STATUS(pub i32);
pub const WEB_SOCKET_SUCCESS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1000i32);
pub const WEB_SOCKET_ENDPOINT_UNAVAILABLE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1001i32);
pub const WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1002i32);
pub const WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1003i32);
pub const WEB_SOCKET_EMPTY_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1005i32);
pub const WEB_SOCKET_ABORTED_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1006i32);
pub const WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1007i32);
pub const WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1008i32);
pub const WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1009i32);
pub const WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1010i32);
pub const WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1011i32);
pub const WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1015i32);
impl ::core::convert::From<i32> for WEB_SOCKET_CLOSE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_CLOSE_STATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct WEB_SOCKET_HANDLE(pub isize);
impl ::core::default::Default for WEB_SOCKET_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for WEB_SOCKET_HANDLE {}
unsafe impl ::windows::core::Abi for WEB_SOCKET_HANDLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEB_SOCKET_HTTP_HEADER {
    pub pcName: super::super::Foundation::PSTR,
    pub ulNameLength: u32,
    pub pcValue: super::super::Foundation::PSTR,
    pub ulValueLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WEB_SOCKET_HTTP_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEB_SOCKET_HTTP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEB_SOCKET_HTTP_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WEB_SOCKET_HTTP_HEADER").field("pcName", &self.pcName).field("ulNameLength", &self.ulNameLength).field("pcValue", &self.pcValue).field("ulValueLength", &self.ulValueLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEB_SOCKET_HTTP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.pcName == other.pcName && self.ulNameLength == other.ulNameLength && self.pcValue == other.pcValue && self.ulValueLength == other.ulValueLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEB_SOCKET_HTTP_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WEB_SOCKET_HTTP_HEADER {
    type Abi = Self;
}
pub const WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WEB_SOCKET_PROPERTY {
    pub Type: WEB_SOCKET_PROPERTY_TYPE,
    pub pvValue: *mut ::core::ffi::c_void,
    pub ulValueSize: u32,
}
impl WEB_SOCKET_PROPERTY {}
impl ::core::default::Default for WEB_SOCKET_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_PROPERTY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WEB_SOCKET_PROPERTY").field("Type", &self.Type).field("pvValue", &self.pvValue).field("ulValueSize", &self.ulValueSize).finish()
    }
}
impl ::core::cmp::PartialEq for WEB_SOCKET_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pvValue == other.pvValue && self.ulValueSize == other.ulValueSize
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_PROPERTY {}
unsafe impl ::windows::core::Abi for WEB_SOCKET_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WEB_SOCKET_PROPERTY_TYPE(pub i32);
pub const WEB_SOCKET_RECEIVE_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(0i32);
pub const WEB_SOCKET_SEND_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(1i32);
pub const WEB_SOCKET_DISABLE_MASKING_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(2i32);
pub const WEB_SOCKET_ALLOCATED_BUFFER_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(3i32);
pub const WEB_SOCKET_DISABLE_UTF8_VERIFICATION_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(4i32);
pub const WEB_SOCKET_KEEPALIVE_INTERVAL_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(5i32);
pub const WEB_SOCKET_SUPPORTED_VERSIONS_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(6i32);
impl ::core::convert::From<i32> for WEB_SOCKET_PROPERTY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_PROPERTY_TYPE {
    type Abi = Self;
}
#[inline]
pub unsafe fn WebSocketAbortHandle<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketAbortHandle(hwebsocket: WEB_SOCKET_HANDLE);
        }
        ::core::mem::transmute(WebSocketAbortHandle(hwebsocket.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebSocketBeginClientHandshake<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0, pszsubprotocols: *const super::super::Foundation::PSTR, ulsubprotocolcount: u32, pszextensions: *const super::super::Foundation::PSTR, ulextensioncount: u32, pinitialheaders: *const WEB_SOCKET_HTTP_HEADER, ulinitialheadercount: u32, padditionalheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketBeginClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocols: *const super::super::Foundation::PSTR, ulsubprotocolcount: u32, pszextensions: *const super::super::Foundation::PSTR, ulextensioncount: u32, pinitialheaders: *const WEB_SOCKET_HTTP_HEADER, ulinitialheadercount: u32, padditionalheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut u32) -> ::windows::core::HRESULT;
        }
        WebSocketBeginClientHandshake(
            hwebsocket.into_param().abi(),
            ::core::mem::transmute(pszsubprotocols),
            ::core::mem::transmute(ulsubprotocolcount),
            ::core::mem::transmute(pszextensions),
            ::core::mem::transmute(ulextensioncount),
            ::core::mem::transmute(pinitialheaders),
            ::core::mem::transmute(ulinitialheadercount),
            ::core::mem::transmute(padditionalheaders),
            ::core::mem::transmute(puladditionalheadercount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebSocketBeginServerHandshake<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(
    hwebsocket: Param0,
    pszsubprotocolselected: Param1,
    pszextensionselected: *const super::super::Foundation::PSTR,
    ulextensionselectedcount: u32,
    prequestheaders: *const WEB_SOCKET_HTTP_HEADER,
    ulrequestheadercount: u32,
    presponseheaders: *mut *mut WEB_SOCKET_HTTP_HEADER,
    pulresponseheadercount: *mut u32,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketBeginServerHandshake(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocolselected: super::super::Foundation::PSTR, pszextensionselected: *const super::super::Foundation::PSTR, ulextensionselectedcount: u32, prequestheaders: *const WEB_SOCKET_HTTP_HEADER, ulrequestheadercount: u32, presponseheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount: *mut u32) -> ::windows::core::HRESULT;
        }
        WebSocketBeginServerHandshake(
            hwebsocket.into_param().abi(),
            pszsubprotocolselected.into_param().abi(),
            ::core::mem::transmute(pszextensionselected),
            ::core::mem::transmute(ulextensionselectedcount),
            ::core::mem::transmute(prequestheaders),
            ::core::mem::transmute(ulrequestheadercount),
            ::core::mem::transmute(presponseheaders),
            ::core::mem::transmute(pulresponseheadercount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketCompleteAction<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0, pvactioncontext: *const ::core::ffi::c_void, ulbytestransferred: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketCompleteAction(hwebsocket: WEB_SOCKET_HANDLE, pvactioncontext: *const ::core::ffi::c_void, ulbytestransferred: u32);
        }
        ::core::mem::transmute(WebSocketCompleteAction(hwebsocket.into_param().abi(), ::core::mem::transmute(pvactioncontext), ::core::mem::transmute(ulbytestransferred)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketCreateClientHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32) -> ::windows::core::Result<WEB_SOCKET_HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketCreateClientHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32, phwebsocket: *mut WEB_SOCKET_HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: <WEB_SOCKET_HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WebSocketCreateClientHandle(::core::mem::transmute(pproperties), ::core::mem::transmute(ulpropertycount), &mut result__).from_abi::<WEB_SOCKET_HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketCreateServerHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32) -> ::windows::core::Result<WEB_SOCKET_HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketCreateServerHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32, phwebsocket: *mut WEB_SOCKET_HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: <WEB_SOCKET_HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WebSocketCreateServerHandle(::core::mem::transmute(pproperties), ::core::mem::transmute(ulpropertycount), &mut result__).from_abi::<WEB_SOCKET_HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketDeleteHandle<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketDeleteHandle(hwebsocket: WEB_SOCKET_HANDLE);
        }
        ::core::mem::transmute(WebSocketDeleteHandle(hwebsocket.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebSocketEndClientHandshake<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0, presponseheaders: *const WEB_SOCKET_HTTP_HEADER, ulreponseheadercount: u32, pulselectedextensions: *mut u32, pulselectedextensioncount: *mut u32, pulselectedsubprotocol: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketEndClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, presponseheaders: *const WEB_SOCKET_HTTP_HEADER, ulreponseheadercount: u32, pulselectedextensions: *mut u32, pulselectedextensioncount: *mut u32, pulselectedsubprotocol: *mut u32) -> ::windows::core::HRESULT;
        }
        WebSocketEndClientHandshake(hwebsocket.into_param().abi(), ::core::mem::transmute(presponseheaders), ::core::mem::transmute(ulreponseheadercount), ::core::mem::transmute(pulselectedextensions), ::core::mem::transmute(pulselectedextensioncount), ::core::mem::transmute(pulselectedsubprotocol)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketEndServerHandshake<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketEndServerHandshake(hwebsocket: WEB_SOCKET_HANDLE) -> ::windows::core::HRESULT;
        }
        WebSocketEndServerHandshake(hwebsocket.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketGetAction<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut u32, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: *mut *mut ::core::ffi::c_void, pvactioncontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketGetAction(hwebsocket: WEB_SOCKET_HANDLE, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut u32, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: *mut *mut ::core::ffi::c_void, pvactioncontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WebSocketGetAction(
            hwebsocket.into_param().abi(),
            ::core::mem::transmute(eactionqueue),
            ::core::mem::transmute(pdatabuffers),
            ::core::mem::transmute(puldatabuffercount),
            ::core::mem::transmute(paction),
            ::core::mem::transmute(pbuffertype),
            ::core::mem::transmute(pvapplicationcontext),
            ::core::mem::transmute(pvactioncontext),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: *mut ::core::ffi::c_void, ulsize: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: *mut ::core::ffi::c_void, ulsize: *mut u32) -> ::windows::core::HRESULT;
        }
        WebSocketGetGlobalProperty(::core::mem::transmute(etype), ::core::mem::transmute(pvvalue), ::core::mem::transmute(ulsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketReceive<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0, pbuffer: *const WEB_SOCKET_BUFFER, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketReceive(hwebsocket: WEB_SOCKET_HANDLE, pbuffer: *const WEB_SOCKET_BUFFER, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WebSocketReceive(hwebsocket.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WebSocketSend<'a, Param0: ::windows::core::IntoParam<'a, WEB_SOCKET_HANDLE>>(hwebsocket: Param0, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: *const WEB_SOCKET_BUFFER, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebSocketSend(hwebsocket: WEB_SOCKET_HANDLE, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: *const WEB_SOCKET_BUFFER, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WebSocketSend(hwebsocket.into_param().abi(), ::core::mem::transmute(buffertype), ::core::mem::transmute(pbuffer), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
