#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
#[inline]
pub unsafe fn JsAddRef(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsAddRef(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
        }
        ::core::mem::transmute(JsAddRef(::core::mem::transmute(r#ref), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type JsBackgroundWorkItemCallback = unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void);
pub type JsBeforeCollectCallback = unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void);
#[inline]
pub unsafe fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsBoolToBoolean(::core::mem::transmute(value), ::core::mem::transmute(booleanvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsBooleanToBool(value: *const ::core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsBooleanToBool(value: *const ::core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsBooleanToBool(::core::mem::transmute(value), ::core::mem::transmute(boolvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCallFunction(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCallFunction(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCallFunction(::core::mem::transmute(function), ::core::mem::transmute(arguments), ::core::mem::transmute(argumentcount), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCollectGarbage(runtime: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCollectGarbage(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCollectGarbage(::core::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConstructObject(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConstructObject(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsConstructObject(::core::mem::transmute(function), ::core::mem::transmute(arguments), ::core::mem::transmute(argumentcount), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConvertValueToBoolean(value: *const ::core::ffi::c_void, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConvertValueToBoolean(value: *const ::core::ffi::c_void, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsConvertValueToBoolean(::core::mem::transmute(value), ::core::mem::transmute(booleanvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConvertValueToNumber(value: *const ::core::ffi::c_void, numbervalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConvertValueToNumber(value: *const ::core::ffi::c_void, numbervalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsConvertValueToNumber(::core::mem::transmute(value), ::core::mem::transmute(numbervalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConvertValueToObject(value: *const ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConvertValueToObject(value: *const ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsConvertValueToObject(::core::mem::transmute(value), ::core::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConvertValueToString(value: *const ::core::ffi::c_void, stringvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConvertValueToString(value: *const ::core::ffi::c_void, stringvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsConvertValueToString(::core::mem::transmute(value), ::core::mem::transmute(stringvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateArray(length: u32, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateArray(length: u32, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateArray(::core::mem::transmute(length), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsCreateContext<'a, Param1: ::windows::core::IntoParam<'a, super::Diagnostics::Debug::IDebugApplication64>>(runtime: *const ::core::ffi::c_void, debugapplication: Param1, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateContext(runtime: *const ::core::ffi::c_void, debugapplication: ::windows::core::RawPtr, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateContext(::core::mem::transmute(runtime), debugapplication.into_param().abi(), ::core::mem::transmute(newcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsCreateContext<'a, Param1: ::windows::core::IntoParam<'a, super::Diagnostics::Debug::IDebugApplication32>>(runtime: *const ::core::ffi::c_void, debugapplication: Param1, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateContext(runtime: *const ::core::ffi::c_void, debugapplication: ::windows::core::RawPtr, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateContext(::core::mem::transmute(runtime), debugapplication.into_param().abi(), ::core::mem::transmute(newcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateError(::core::mem::transmute(message), ::core::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateExternalObject(data: *const ::core::ffi::c_void, finalizecallback: ::core::option::Option<JsFinalizeCallback>, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateExternalObject(data: *const ::core::ffi::c_void, finalizecallback: ::windows::core::RawPtr, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateExternalObject(::core::mem::transmute(data), ::core::mem::transmute(finalizecallback), ::core::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateFunction(nativefunction: ::core::option::Option<JsNativeFunction>, callbackstate: *const ::core::ffi::c_void, function: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateFunction(nativefunction: ::windows::core::RawPtr, callbackstate: *const ::core::ffi::c_void, function: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateFunction(::core::mem::transmute(nativefunction), ::core::mem::transmute(callbackstate), ::core::mem::transmute(function)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateObject(object: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateObject(object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateObject(::core::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateRangeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateRangeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateRangeError(::core::mem::transmute(message), ::core::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateReferenceError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateReferenceError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateReferenceError(::core::mem::transmute(message), ::core::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: ::core::option::Option<JsThreadServiceCallback>, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: ::windows::core::RawPtr, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateRuntime(::core::mem::transmute(attributes), ::core::mem::transmute(runtimeversion), ::core::mem::transmute(threadservice), ::core::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateSyntaxError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateSyntaxError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateSyntaxError(::core::mem::transmute(message), ::core::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateTypeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateTypeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateTypeError(::core::mem::transmute(message), ::core::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateURIError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateURIError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsCreateURIError(::core::mem::transmute(message), ::core::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDefineProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDefineProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsDefineProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(propertydescriptor), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDeleteIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDeleteIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsDeleteIndexedProperty(::core::mem::transmute(object), ::core::mem::transmute(index)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDeleteProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, usestrictrules: u8, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDeleteProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, usestrictrules: u8, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsDeleteProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(usestrictrules), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDisableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDisableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsDisableRuntimeExecution(::core::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDisposeRuntime(runtime: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDisposeRuntime(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsDisposeRuntime(::core::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsDoubleToNumber(::core::mem::transmute(doublevalue), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsEnableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsEnableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsEnableRuntimeExecution(::core::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsEnumerateHeap(enumerator: *mut ::core::option::Option<super::Diagnostics::Debug::IActiveScriptProfilerHeapEnum>) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsEnumerateHeap(enumerator: *mut ::windows::core::RawPtr) -> JsErrorCode;
        }
        ::core::mem::transmute(JsEnumerateHeap(::core::mem::transmute(enumerator)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsEquals(::core::mem::transmute(object1), ::core::mem::transmute(object2), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JsErrorCode(pub u32);
pub const JsNoError: JsErrorCode = JsErrorCode(0u32);
pub const JsErrorCategoryUsage: JsErrorCode = JsErrorCode(65536u32);
pub const JsErrorInvalidArgument: JsErrorCode = JsErrorCode(65537u32);
pub const JsErrorNullArgument: JsErrorCode = JsErrorCode(65538u32);
pub const JsErrorNoCurrentContext: JsErrorCode = JsErrorCode(65539u32);
pub const JsErrorInExceptionState: JsErrorCode = JsErrorCode(65540u32);
pub const JsErrorNotImplemented: JsErrorCode = JsErrorCode(65541u32);
pub const JsErrorWrongThread: JsErrorCode = JsErrorCode(65542u32);
pub const JsErrorRuntimeInUse: JsErrorCode = JsErrorCode(65543u32);
pub const JsErrorBadSerializedScript: JsErrorCode = JsErrorCode(65544u32);
pub const JsErrorInDisabledState: JsErrorCode = JsErrorCode(65545u32);
pub const JsErrorCannotDisableExecution: JsErrorCode = JsErrorCode(65546u32);
pub const JsErrorHeapEnumInProgress: JsErrorCode = JsErrorCode(65547u32);
pub const JsErrorArgumentNotObject: JsErrorCode = JsErrorCode(65548u32);
pub const JsErrorInProfileCallback: JsErrorCode = JsErrorCode(65549u32);
pub const JsErrorInThreadServiceCallback: JsErrorCode = JsErrorCode(65550u32);
pub const JsErrorCannotSerializeDebugScript: JsErrorCode = JsErrorCode(65551u32);
pub const JsErrorAlreadyDebuggingContext: JsErrorCode = JsErrorCode(65552u32);
pub const JsErrorAlreadyProfilingContext: JsErrorCode = JsErrorCode(65553u32);
pub const JsErrorIdleNotEnabled: JsErrorCode = JsErrorCode(65554u32);
pub const JsErrorCategoryEngine: JsErrorCode = JsErrorCode(131072u32);
pub const JsErrorOutOfMemory: JsErrorCode = JsErrorCode(131073u32);
pub const JsErrorCategoryScript: JsErrorCode = JsErrorCode(196608u32);
pub const JsErrorScriptException: JsErrorCode = JsErrorCode(196609u32);
pub const JsErrorScriptCompile: JsErrorCode = JsErrorCode(196610u32);
pub const JsErrorScriptTerminated: JsErrorCode = JsErrorCode(196611u32);
pub const JsErrorScriptEvalDisabled: JsErrorCode = JsErrorCode(196612u32);
pub const JsErrorCategoryFatal: JsErrorCode = JsErrorCode(262144u32);
pub const JsErrorFatal: JsErrorCode = JsErrorCode(262145u32);
impl ::core::convert::From<u32> for JsErrorCode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JsErrorCode {
    type Abi = Self;
}
impl ::core::ops::BitOr for JsErrorCode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JsErrorCode {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JsErrorCode {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JsErrorCode {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JsErrorCode {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub type JsFinalizeCallback = unsafe extern "system" fn(data: *const ::core::ffi::c_void);
#[inline]
pub unsafe fn JsGetAndClearException(exception: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetAndClearException(exception: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetAndClearException(::core::mem::transmute(exception)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetCurrentContext(currentcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetCurrentContext(currentcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetCurrentContext(::core::mem::transmute(currentcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetExtensionAllowed(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetExtensionAllowed(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetExtensionAllowed(::core::mem::transmute(object), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetExternalData(object: *const ::core::ffi::c_void, externaldata: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetExternalData(object: *const ::core::ffi::c_void, externaldata: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetExternalData(::core::mem::transmute(object), ::core::mem::transmute(externaldata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetFalseValue(falsevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetFalseValue(falsevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetFalseValue(::core::mem::transmute(falsevalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetGlobalObject(globalobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetGlobalObject(globalobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetGlobalObject(::core::mem::transmute(globalobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetIndexedProperty(::core::mem::transmute(object), ::core::mem::transmute(index), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetNullValue(nullvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetNullValue(nullvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetNullValue(::core::mem::transmute(nullvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetOwnPropertyDescriptor(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetOwnPropertyDescriptor(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetOwnPropertyDescriptor(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(propertydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetOwnPropertyNames(object: *const ::core::ffi::c_void, propertynames: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetOwnPropertyNames(object: *const ::core::ffi::c_void, propertynames: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetOwnPropertyNames(::core::mem::transmute(object), ::core::mem::transmute(propertynames)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsGetPropertyIdFromName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(name: Param0, propertyid: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetPropertyIdFromName(name: super::super::Foundation::PWSTR, propertyid: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetPropertyIdFromName(name.into_param().abi(), ::core::mem::transmute(propertyid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetPropertyNameFromId(propertyid: *const ::core::ffi::c_void, name: *mut *mut u16) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetPropertyNameFromId(propertyid: *const ::core::ffi::c_void, name: *mut *mut u16) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetPropertyNameFromId(::core::mem::transmute(propertyid), ::core::mem::transmute(name)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetPrototype(::core::mem::transmute(object), ::core::mem::transmute(prototypeobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetRuntime(context: *const ::core::ffi::c_void, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetRuntime(context: *const ::core::ffi::c_void, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetRuntime(::core::mem::transmute(context), ::core::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetRuntimeMemoryLimit(::core::mem::transmute(runtime), ::core::mem::transmute(memorylimit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetRuntimeMemoryUsage(runtime: *const ::core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetRuntimeMemoryUsage(runtime: *const ::core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetRuntimeMemoryUsage(::core::mem::transmute(runtime), ::core::mem::transmute(memoryusage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetStringLength(stringvalue: *const ::core::ffi::c_void, length: *mut i32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetStringLength(stringvalue: *const ::core::ffi::c_void, length: *mut i32) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetStringLength(::core::mem::transmute(stringvalue), ::core::mem::transmute(length)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetTrueValue(truevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetTrueValue(truevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetTrueValue(::core::mem::transmute(truevalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetUndefinedValue(::core::mem::transmute(undefinedvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetValueType(value: *const ::core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetValueType(value: *const ::core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode;
        }
        ::core::mem::transmute(JsGetValueType(::core::mem::transmute(value), ::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsHasException(hasexception: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsHasException(hasexception: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsHasException(::core::mem::transmute(hasexception)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsHasExternalData(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsHasExternalData(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsHasExternalData(::core::mem::transmute(object), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsHasIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsHasIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsHasIndexedProperty(::core::mem::transmute(object), ::core::mem::transmute(index), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsHasProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsHasProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsHasProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(hasproperty)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsIdle(nextidletick: *mut u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsIdle(nextidletick: *mut u32) -> JsErrorCode;
        }
        ::core::mem::transmute(JsIdle(::core::mem::transmute(nextidletick)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsIntToNumber(intvalue: i32, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsIntToNumber(intvalue: i32, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsIntToNumber(::core::mem::transmute(intvalue), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsIsEnumeratingHeap(::core::mem::transmute(isenumeratingheap)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsIsRuntimeExecutionDisabled(runtime: *const ::core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsIsRuntimeExecutionDisabled(runtime: *const ::core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsIsRuntimeExecutionDisabled(::core::mem::transmute(runtime), ::core::mem::transmute(isdisabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type JsMemoryAllocationCallback = unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, allocationevent: JsMemoryEventType, allocationsize: usize) -> bool;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JsMemoryEventType(pub i32);
pub const JsMemoryAllocate: JsMemoryEventType = JsMemoryEventType(0i32);
pub const JsMemoryFree: JsMemoryEventType = JsMemoryEventType(1i32);
pub const JsMemoryFailure: JsMemoryEventType = JsMemoryEventType(2i32);
impl ::core::convert::From<i32> for JsMemoryEventType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JsMemoryEventType {
    type Abi = Self;
}
pub type JsNativeFunction = unsafe extern "system" fn(callee: *const ::core::ffi::c_void, isconstructcall: bool, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, callbackstate: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[inline]
pub unsafe fn JsNumberToDouble(value: *const ::core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsNumberToDouble(value: *const ::core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode;
        }
        ::core::mem::transmute(JsNumberToDouble(::core::mem::transmute(value), ::core::mem::transmute(doublevalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsParseScript<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, sourcecontext: usize, sourceurl: Param2, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsParseScript(script: super::super::Foundation::PWSTR, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsParseScript(script.into_param().abi(), ::core::mem::transmute(sourcecontext), sourceurl.into_param().abi(), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsParseSerializedScript<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, buffer: *const u8, sourcecontext: usize, sourceurl: Param3, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsParseSerializedScript(script: super::super::Foundation::PWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsParseSerializedScript(script.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(sourcecontext), sourceurl.into_param().abi(), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsPointerToString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(stringvalue: Param0, stringlength: usize, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsPointerToString(stringvalue: super::super::Foundation::PWSTR, stringlength: usize, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsPointerToString(stringvalue.into_param().abi(), ::core::mem::transmute(stringlength), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsPreventExtension(object: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsPreventExtension(object: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsPreventExtension(::core::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsRelease(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsRelease(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
        }
        ::core::mem::transmute(JsRelease(::core::mem::transmute(r#ref), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsRunScript<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, sourcecontext: usize, sourceurl: Param2, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsRunScript(script: super::super::Foundation::PWSTR, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsRunScript(script.into_param().abi(), ::core::mem::transmute(sourcecontext), sourceurl.into_param().abi(), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsRunSerializedScript<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, buffer: *const u8, sourcecontext: usize, sourceurl: Param3, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsRunSerializedScript(script: super::super::Foundation::PWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsRunSerializedScript(script.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(sourcecontext), sourceurl.into_param().abi(), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JsRuntimeAttributes(pub i32);
pub const JsRuntimeAttributeNone: JsRuntimeAttributes = JsRuntimeAttributes(0i32);
pub const JsRuntimeAttributeDisableBackgroundWork: JsRuntimeAttributes = JsRuntimeAttributes(1i32);
pub const JsRuntimeAttributeAllowScriptInterrupt: JsRuntimeAttributes = JsRuntimeAttributes(2i32);
pub const JsRuntimeAttributeEnableIdleProcessing: JsRuntimeAttributes = JsRuntimeAttributes(4i32);
pub const JsRuntimeAttributeDisableNativeCodeGeneration: JsRuntimeAttributes = JsRuntimeAttributes(8i32);
pub const JsRuntimeAttributeDisableEval: JsRuntimeAttributes = JsRuntimeAttributes(16i32);
impl ::core::convert::From<i32> for JsRuntimeAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JsRuntimeAttributes {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JsRuntimeVersion(pub i32);
pub const JsRuntimeVersion10: JsRuntimeVersion = JsRuntimeVersion(0i32);
pub const JsRuntimeVersion11: JsRuntimeVersion = JsRuntimeVersion(1i32);
pub const JsRuntimeVersionEdge: JsRuntimeVersion = JsRuntimeVersion(-1i32);
impl ::core::convert::From<i32> for JsRuntimeVersion {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JsRuntimeVersion {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsSerializeScript<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSerializeScript(script: super::super::Foundation::PWSTR, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSerializeScript(script.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetCurrentContext(context: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetCurrentContext(context: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetCurrentContext(::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetException(exception: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetException(exception: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetException(::core::mem::transmute(exception)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetExternalData(object: *const ::core::ffi::c_void, externaldata: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetExternalData(object: *const ::core::ffi::c_void, externaldata: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetExternalData(::core::mem::transmute(object), ::core::mem::transmute(externaldata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetIndexedProperty(::core::mem::transmute(object), ::core::mem::transmute(index), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, usestrictrules: u8) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, usestrictrules: u8) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(value), ::core::mem::transmute(usestrictrules)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *const ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetPrototype(::core::mem::transmute(object), ::core::mem::transmute(prototypeobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetRuntimeBeforeCollectCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, beforecollectcallback: ::core::option::Option<JsBeforeCollectCallback>) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetRuntimeBeforeCollectCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, beforecollectcallback: ::windows::core::RawPtr) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetRuntimeBeforeCollectCallback(::core::mem::transmute(runtime), ::core::mem::transmute(callbackstate), ::core::mem::transmute(beforecollectcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetRuntimeMemoryAllocationCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, allocationcallback: ::core::option::Option<JsMemoryAllocationCallback>) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetRuntimeMemoryAllocationCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, allocationcallback: ::windows::core::RawPtr) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetRuntimeMemoryAllocationCallback(::core::mem::transmute(runtime), ::core::mem::transmute(callbackstate), ::core::mem::transmute(allocationcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: usize) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: usize) -> JsErrorCode;
        }
        ::core::mem::transmute(JsSetRuntimeMemoryLimit(::core::mem::transmute(runtime), ::core::mem::transmute(memorylimit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsStartDebugging<'a, Param0: ::windows::core::IntoParam<'a, super::Diagnostics::Debug::IDebugApplication64>>(debugapplication: Param0) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStartDebugging(debugapplication: ::windows::core::RawPtr) -> JsErrorCode;
        }
        ::core::mem::transmute(JsStartDebugging(debugapplication.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsStartDebugging<'a, Param0: ::windows::core::IntoParam<'a, super::Diagnostics::Debug::IDebugApplication32>>(debugapplication: Param0) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStartDebugging(debugapplication: ::windows::core::RawPtr) -> JsErrorCode;
        }
        ::core::mem::transmute(JsStartDebugging(debugapplication.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsStartProfiling<'a, Param0: ::windows::core::IntoParam<'a, super::Diagnostics::Debug::IActiveScriptProfilerCallback>>(callback: Param0, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStartProfiling(callback: ::windows::core::RawPtr, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode;
        }
        ::core::mem::transmute(JsStartProfiling(callback.into_param().abi(), ::core::mem::transmute(eventmask), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsStopProfiling(reason: ::windows::core::HRESULT) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStopProfiling(reason: ::windows::core::HRESULT) -> JsErrorCode;
        }
        ::core::mem::transmute(JsStopProfiling(::core::mem::transmute(reason)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsStrictEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStrictEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
        }
        ::core::mem::transmute(JsStrictEquals(::core::mem::transmute(object1), ::core::mem::transmute(object2), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsStringToPointer(value: *const ::core::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStringToPointer(value: *const ::core::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode;
        }
        ::core::mem::transmute(JsStringToPointer(::core::mem::transmute(value), ::core::mem::transmute(stringvalue), ::core::mem::transmute(stringlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type JsThreadServiceCallback = unsafe extern "system" fn(callback: ::windows::core::RawPtr, callbackstate: *const ::core::ffi::c_void) -> bool;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn JsValueToVariant(object: *const ::core::ffi::c_void, variant: *mut super::Com::VARIANT) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsValueToVariant(object: *const ::core::ffi::c_void, variant: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> JsErrorCode;
        }
        ::core::mem::transmute(JsValueToVariant(::core::mem::transmute(object), ::core::mem::transmute(variant)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JsValueType(pub i32);
pub const JsUndefined: JsValueType = JsValueType(0i32);
pub const JsNull: JsValueType = JsValueType(1i32);
pub const JsNumber: JsValueType = JsValueType(2i32);
pub const JsString: JsValueType = JsValueType(3i32);
pub const JsBoolean: JsValueType = JsValueType(4i32);
pub const JsObject: JsValueType = JsValueType(5i32);
pub const JsFunction: JsValueType = JsValueType(6i32);
pub const JsError: JsValueType = JsValueType(7i32);
pub const JsArray: JsValueType = JsValueType(8i32);
impl ::core::convert::From<i32> for JsValueType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JsValueType {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn JsVariantToValue(variant: *const super::Com::VARIANT, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsVariantToValue(variant: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
        }
        ::core::mem::transmute(JsVariantToValue(::core::mem::transmute(variant), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
