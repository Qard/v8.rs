#![allow(non_snake_case)]
// #![allow(non_uppercase_statics)]
#![allow(non_upper_case_globals)]
#![allow(raw_pointer_derive)]
#![feature(libc)]

extern crate libc;

use std::default::Default;
use std::fmt;
use std::mem;
use std::ptr;

#[cfg(target_pointer_width="32")] const kApiPointerSize: usize = 4;
#[cfg(target_pointer_width="64")] const kApiPointerSize: usize = 8;

const kApiInt64Size: usize = 8;
const kAmountOfExternalAllocatedMemoryOffset: usize = 4 * kApiPointerSize;
const kAmountOfExternalAllocatedMemoryAtLastGlobalGCOffset: usize =
        kAmountOfExternalAllocatedMemoryOffset + kApiInt64Size;
const kIsolateRootsOffset: usize =
        kAmountOfExternalAllocatedMemoryAtLastGlobalGCOffset + kApiInt64Size +
        kApiPointerSize;
const kUndefinedValueRootIndex: usize = 5;
const kNullValueRootIndex: usize = 7;
const kTrueValueRootIndex: usize = 8;
const kFalseValueRootIndex: usize = 9;
const kEmptyStringRootIndex: usize = 152;

#[link(name="v8")]
extern {
    fn _ZN2v87Context4ExitEv(this: Context);
    fn _ZN2v87Context5EnterEv(this: Context);
    fn _ZN2v87Context6GlobalEv(this: Context) -> Object;
    fn _ZN2v87Context3NewEPNS_7IsolateEPNS_22ExtensionConfigurationENS_6HandleINS_14ObjectTemplateEEENS5_INS_5ValueEEE(
            isolate: Isolate, extensions: *mut ExtensionConfiguration,
            global_template: ObjectTemplate, global_object: Value) -> Context;
    fn _ZN2v88Function4CallENS_6HandleINS_5ValueEEEiPS3_(this: Function,
                                                         recv: Value,
                                                         argc: i32,
                                                         argv: *const Value)
                                                         -> Value;
    fn _ZNK2v820FunctionCallbackInfoINS_5ValueEE14GetReturnValueEv(
            this: FunctionCallbackInfo) -> ReturnValue;
    fn _ZNK2v820FunctionCallbackInfoINS_5ValueEEixEi(this: FunctionCallbackInfo,
                                                     index: i32) -> Value;
    fn _ZNK2v820FunctionCallbackInfoINS_5ValueEE10GetIsolateEv(
            this: FunctionCallbackInfo) -> Isolate;
    fn _ZN2v816FunctionTemplate11GetFunctionEv(this: FunctionTemplate)
                                               -> Function;
    fn _ZN2v816FunctionTemplate3NewEPNS_7IsolateEPFvRKNS_20FunctionCallbackInfoINS_5ValueEEEENS_6HandleIS4_EENSA_INS_9SignatureEEEi(
            isolate: Isolate, callback: Option<FunctionCallback>,
            data: Value, signature: Signature, length: i32) -> FunctionTemplate;
    fn _ZN2v811HandleScopeC1EPNS_7IsolateE(this: &mut HandleScope,
                                           isolate: Isolate);
    fn _ZN2v811HandleScopeD1Ev(this: &mut HandleScope);
    fn _ZN2v87Integer15NewFromUnsignedEPNS_7IsolateEj(isolate: Isolate,
                                                      value: u32) -> Integer;
    fn _ZN2v87Integer3NewEPNS_7IsolateEi(isolate: Isolate,
                                         value: i32) -> Integer;
    fn _ZNK2v87Integer5ValueEv(this: Integer) -> i64;
    fn _ZN2v87Isolate3NewERKNS0_12CreateParamsE(params: &CreateParams)
                                                -> Isolate;
    fn _ZN2v87Isolate7DisposeEv(this: Isolate);
    fn _ZN2v87Isolate5EnterEv(this: Isolate);
    fn _ZN2v87Isolate4ExitEv(this: Isolate);
    fn _ZN2v86Locker10InitializeEPNS_7IsolateE(this: &mut Locker,
                                               isolate: Isolate);
    fn _ZN2v86Locker8IsActiveEv() -> bool;
    fn _ZN2v86Locker8IsLockedEPNS_7IsolateE(isolate: Isolate) -> bool;
    fn _ZN2v86LockerD1Ev(this: &mut Locker);
    fn _ZN2v86Number3NewEPNS_7IsolateEd(isolate: Isolate, value: f64) -> Number;
    fn _ZN2v86Object3GetEj(this: Object, index: u32) -> Value;
    fn _ZN2v86Object3GetENS_6HandleINS_5ValueEEE(this: Object,
                                                 key: Value) -> Value;
    fn _ZN2v86Object3NewEPNS_7IsolateE(isolate: Isolate) -> Object;
    fn _ZN2v86Object3SetEjNS_6HandleINS_5ValueEEE(this: Object, key: u32,
                                                  value: Value) -> bool;
    fn _ZN2v86Object3SetENS_6HandleINS_5ValueEEES3_(this: Object, key: Value,
                                                    value: Value) -> bool;
    fn _ZN2v86Script7CompileENS_6HandleINS_6StringEEEPNS_12ScriptOriginE(
            source: String, origin: *mut ScriptOrigin) -> Script;
    fn _ZN2v86Script3RunEv(this: Script) -> Value;
    fn _ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS0_13NewStringTypeEi(
            isolate: Isolate, data: *const u8,
            typ: NewStringType, length: i32) -> String;
    fn _ZNK2v86String6LengthEv(this: String) -> i32;
    fn _ZN2v88Unlocker10InitializeEPNS_7IsolateE(this: &mut Unlocker,
                                                 isolate: Isolate);
    fn _ZN2v88UnlockerD1Ev(this: &mut Unlocker);
    fn _ZN2v82V810InitializeEv() -> bool;
    fn _ZN2v82V87DisposeEv() -> bool;
    fn _ZNK2v85Value10IsDataViewEv(this: Value) -> bool;
    fn _ZNK2v85Value10IsExternalEv(this: Value) -> bool;
    fn _ZNK2v85Value10IsFunctionEv(this: Value) -> bool;
    fn _ZNK2v85Value11IsInt8ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value12IsInt16ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value12IsInt32ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value12IsTypedArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value12IsUint8ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value13IsArrayBufferEv(this: Value) -> bool;
    fn _ZNK2v85Value13IsNativeErrorEv(this: Value) -> bool;
    fn _ZNK2v85Value13IsUint16ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value13IsUint32ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsFloat32ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsFloat64ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsNumberObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsStringObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsSymbolObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value15IsBooleanObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value17IsArgumentsObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value17IsArrayBufferViewEv(this: Value) -> bool;
    fn _ZNK2v85Value17IsGeneratorObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value19IsGeneratorFunctionEv(this: Value) -> bool;
    fn _ZNK2v85Value19IsUint8ClampedArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value5IsMapEv(this: Value) -> bool;
    fn _ZNK2v85Value5IsSetEv(this: Value) -> bool;
    fn _ZNK2v85Value6IsDateEv(this: Value) -> bool;
    fn _ZNK2v85Value6IsNameEv(this: Value) -> bool;
    fn _ZNK2v85Value6IsTrueEv(this: Value) -> bool;
    fn _ZNK2v85Value7IsArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value7IsFalseEv(this: Value) -> bool;
    fn _ZNK2v85Value7IsInt32Ev(this: Value) -> bool;
    fn _ZNK2v85Value8IsNumberEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsRegExpEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsStringEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsSymbolEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsUint32Ev(this: Value) -> bool;
    fn _ZNK2v85Value9IsBooleanEv(this: Value) -> bool;
    fn _ZNK2v85Value9IsPromiseEv(this: Value) -> bool;
    fn _ZNK2v85Value9IsWeakMapEv(this: Value) -> bool;
    fn _ZNK2v85Value9IsWeakSetEv(this: Value) -> bool;
    fn _ZNK2v85Value11QuickIsNullEv(this: Value) -> bool;
    fn _ZNK2v85Value16QuickIsUndefinedEv(this: Value) -> bool;
    fn _ZNK2v85Value10Int32ValueEv(this: Value) -> i32;
    fn _ZNK2v85Value11NumberValueEv(this: Value) -> f64;
    fn _ZNK2v85Value11Uint32ValueEv(this: Value) -> u32;
    fn _ZNK2v85Value12IntegerValueEv(this: Value) -> i64;
}

pub trait IndexT {
    fn get(&self, object: Object) -> Value;
    fn set(&self, object: Object, value: Value) -> bool;
}

pub trait ValueT : IndexT {
    fn as_val(&self) -> Value;
    fn from_val(value: Value) -> Self;
}

macro_rules! data_methods(
    ($ty:ident) => (
        impl $ty {
            #[inline(always)]
            fn raw_ptr(&self) -> *mut $ty {
                match *self {
                    $ty(that) => unsafe { *that }
                }
            }

            #[allow(dead_code)]
            fn to_option(&self) -> Option<$ty> {
                match *self {
                    $ty(that) if that.is_null() => None,
                    _ => Some(*self),
                }
            }
        }

        impl Default for $ty {
            fn default() -> $ty {
                $ty(ptr::null_mut())
            }
        }

        impl PartialEq for $ty {
            fn eq(&self, that: &$ty) -> bool {
                self.raw_ptr() == that.raw_ptr()
            }
        }

        impl fmt::Debug for $ty {
            // TODO(bnoordhuis) Maybe specialize for SMIs and strings.
            // Maybe ToString() objects?
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "{}({:p})", stringify!($ty), self.raw_ptr())
            }
        }
    );
);

macro_rules! value_methods(
    ($ty:ident) => (
        data_methods!($ty);

        impl $ty {
            #[inline(always)]
            pub fn IsArgumentsObject(&self) -> bool {
                unsafe { _ZNK2v85Value17IsArgumentsObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsArray(&self) -> bool {
                unsafe { _ZNK2v85Value7IsArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsArrayBuffer(&self) -> bool {
                unsafe { _ZNK2v85Value13IsArrayBufferEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsArrayBufferView(&self) -> bool {
                unsafe { _ZNK2v85Value17IsArrayBufferViewEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsBoolean(&self) -> bool {
                unsafe { _ZNK2v85Value9IsBooleanEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsBooleanObject(&self) -> bool {
                unsafe { _ZNK2v85Value15IsBooleanObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsDataView(&self) -> bool {
                unsafe { _ZNK2v85Value10IsDataViewEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsDate(&self) -> bool {
                unsafe { _ZNK2v85Value6IsDateEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsExternal(&self) -> bool {
                unsafe { _ZNK2v85Value10IsExternalEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsFalse(&self) -> bool {
                unsafe { _ZNK2v85Value7IsFalseEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsFloat32Array(&self) -> bool {
                unsafe { _ZNK2v85Value14IsFloat32ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsFloat64Array(&self) -> bool {
                unsafe { _ZNK2v85Value14IsFloat64ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsFunction(&self) -> bool {
                unsafe { _ZNK2v85Value10IsFunctionEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsGeneratorFunction(&self) -> bool {
                unsafe { _ZNK2v85Value19IsGeneratorFunctionEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsGeneratorObject(&self) -> bool {
                unsafe { _ZNK2v85Value17IsGeneratorObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsInt16Array(&self) -> bool {
                unsafe { _ZNK2v85Value12IsInt16ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsInt32(&self) -> bool {
                unsafe { _ZNK2v85Value7IsInt32Ev(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsInt32Array(&self) -> bool {
                unsafe { _ZNK2v85Value12IsInt32ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsInt8Array(&self) -> bool {
                unsafe { _ZNK2v85Value11IsInt8ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsMap(&self) -> bool {
                unsafe { _ZNK2v85Value5IsMapEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsName(&self) -> bool {
                unsafe { _ZNK2v85Value6IsNameEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsNativeError(&self) -> bool {
                unsafe { _ZNK2v85Value13IsNativeErrorEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsNull(&self) -> bool {
                // FIXME(bnoordhuis) Use inline heap object tag check.
                unsafe { _ZNK2v85Value11QuickIsNullEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsNumber(&self) -> bool {
                unsafe { _ZNK2v85Value8IsNumberEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsNumberObject(&self) -> bool {
                unsafe { _ZNK2v85Value14IsNumberObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsObject(&self) -> bool {
                unsafe { _ZNK2v85Value8IsObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsPromise(&self) -> bool {
                unsafe { _ZNK2v85Value9IsPromiseEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsRegExp(&self) -> bool {
                unsafe { _ZNK2v85Value8IsRegExpEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsSet(&self) -> bool {
                unsafe { _ZNK2v85Value5IsSetEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsString(&self) -> bool {
                unsafe { _ZNK2v85Value8IsStringEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsStringObject(&self) -> bool {
                unsafe { _ZNK2v85Value14IsStringObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsSymbol(&self) -> bool {
                unsafe { _ZNK2v85Value8IsSymbolEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsSymbolObject(&self) -> bool {
                unsafe { _ZNK2v85Value14IsSymbolObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsTrue(&self) -> bool {
                unsafe { _ZNK2v85Value6IsTrueEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsTypedArray(&self) -> bool {
                unsafe { _ZNK2v85Value12IsTypedArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint16Array(&self) -> bool {
                unsafe { _ZNK2v85Value13IsUint16ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint32(&self) -> bool {
                unsafe { _ZNK2v85Value8IsUint32Ev(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint32Array(&self) -> bool {
                unsafe { _ZNK2v85Value13IsUint32ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint8Array(&self) -> bool {
                unsafe { _ZNK2v85Value12IsUint8ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint8ClampedArray(&self) -> bool {
                unsafe { _ZNK2v85Value19IsUint8ClampedArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUndefined(&self) -> bool {
                // FIXME(bnoordhuis) Use inline heap object tag check.
                unsafe { _ZNK2v85Value16QuickIsUndefinedEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsWeakMap(&self) -> bool {
                unsafe { _ZNK2v85Value9IsWeakMapEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsWeakSet(&self) -> bool {
                unsafe { _ZNK2v85Value9IsWeakSetEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn Int32Value(&self) -> i32 {
                unsafe { _ZNK2v85Value10Int32ValueEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IntegerValue(&self) -> i64 {
                unsafe { _ZNK2v85Value12IntegerValueEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn NumberValue(&self) -> f64 {
                unsafe { _ZNK2v85Value11NumberValueEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn Uint32Value(&self) -> u32 {
                unsafe { _ZNK2v85Value11Uint32ValueEv(self.as_val()) }
            }
            pub fn As<T: ValueT>(&self) -> T {
                ValueT::from_val(self.as_val())
            }
            #[inline(always)]
            fn as_val(&self) -> Value {
                Value(unsafe { mem::transmute(*self) })
            }
        }

        impl IndexT for $ty {
            fn get(&self, object: Object) -> Value {
                unsafe {
                    _ZN2v86Object3GetENS_6HandleINS_5ValueEEE(
                            object, self.as_val())
                }
            }

            fn set(&self, object: Object, value: Value) -> bool {
                unsafe {
                    _ZN2v86Object3SetENS_6HandleINS_5ValueEEES3_(
                            object, self.as_val(), value)
                }
            }
        }

        impl ValueT for $ty {
            #[inline(always)]
            fn as_val(&self) -> Value {
                Value(unsafe { mem::transmute(*self) })
            }
            #[inline(always)]
            fn from_val(value: Value) -> $ty {
                match value {
                    Value(that) => $ty(unsafe { mem::transmute(that) })
                }
            }
        }
    );
);

#[repr(C)]
#[derive(Copy)]
pub struct Boolean(*mut *mut Boolean);

value_methods!(Boolean);

impl Boolean {
    // XXX(bnoordhuis) Never fails but returning Boolean
    // directly is inconsistent with other New() methods.
    pub fn New(isolate: Isolate, value: bool) -> Option<Boolean> {
        Some(match value {
            true => True(isolate),
            false => False(isolate),
        })
    }

    pub fn Value(&self) -> bool {
        self.IsTrue()
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct CreateParams {
    entry_hook: *const u8,
    code_event_handler: *const u8,
    constraints: ResourceConstraints,
    enable_serializer: bool,
}

impl Default for CreateParams {
    fn default() -> CreateParams {
        CreateParams {
            entry_hook: 0 as *const u8,
            code_event_handler: 0 as *const u8,
            constraints: Default::default(),
            enable_serializer: false,
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Context(*mut *mut Context);

data_methods!(Context);

impl Context {
    pub fn Enter(&self) {
        unsafe { _ZN2v87Context5EnterEv(*self) }
    }

    pub fn Exit(&self) {
        unsafe { _ZN2v87Context4ExitEv(*self) }
    }

    pub fn Global(&self) -> Option<Object> {
        unsafe { _ZN2v87Context6GlobalEv(*self) }.to_option()
    }

    pub fn New(isolate: Isolate) -> Option<Context> {
        unsafe {
            _ZN2v87Context3NewEPNS_7IsolateEPNS_22ExtensionConfigurationENS_6HandleINS_14ObjectTemplateEEENS5_INS_5ValueEEE(
                    isolate, ptr::null_mut(), Default::default(),
                    Default::default())
        }.to_option()
    }
}

// Can't use a RAII type for Context::Scope because of
// https://github.com/rust-lang/rust/issues/17858
pub fn with_context_scope<T>(context: Context, closure: fn() -> T) -> T {
    context.Enter();
    let rval = closure();
    context.Exit();
    rval
}

#[repr(C)]
#[derive(Copy)]
pub struct ExtensionConfiguration;

#[repr(C)]
#[derive(Copy)]
pub struct Function(*mut *mut Function);

value_methods!(Function);

impl Function {
    pub fn Call<T: ValueT>(&self, recv: T, argv: &[Value]) -> Option<Value> {
        unsafe {
            _ZN2v88Function4CallENS_6HandleINS_5ValueEEEiPS3_(
                    *self, recv.as_val(), argv.len() as i32,
                    argv.as_slice().as_ptr())
        }.to_option()
    }
}

#[repr(C)]
pub type FunctionCallback = extern fn(FunctionCallbackInfo);

#[repr(C)]
#[derive(Copy)]
pub struct FunctionCallbackInfo(*mut *mut FunctionCallbackInfo);

impl FunctionCallbackInfo {
    pub fn At(&self, index: i32) -> Value {
        unsafe { _ZNK2v820FunctionCallbackInfoINS_5ValueEEixEi(*self, index) }
    }

    pub fn GetIsolate(&self) -> Isolate {
        unsafe { _ZNK2v820FunctionCallbackInfoINS_5ValueEE10GetIsolateEv(*self) }
    }

    pub fn GetReturnValue(&self) -> ReturnValue {
        unsafe {
            _ZNK2v820FunctionCallbackInfoINS_5ValueEE14GetReturnValueEv(*self)
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct FunctionTemplate(*mut *mut FunctionTemplate);

data_methods!(FunctionTemplate);

impl FunctionTemplate {
    pub fn GetFunction(&self) -> Option<Function> {
        unsafe { _ZN2v816FunctionTemplate11GetFunctionEv(*self) }.to_option()
    }

    pub fn New(isolate: Isolate, callback: Option<FunctionCallback>,
               data: Option<Value>, signature: Option<Signature>,
               length: i32) -> Option<FunctionTemplate> {
        let data = data.unwrap_or(Default::default());
        let signature = signature.unwrap_or(Default::default());
        unsafe {
            _ZN2v816FunctionTemplate3NewEPNS_7IsolateEPFvRKNS_20FunctionCallbackInfoINS_5ValueEEEENS_6HandleIS4_EENSA_INS_9SignatureEEEi(
                    isolate, callback, data, signature, length)
        }.to_option()
    }
}

#[repr(C)]
#[derive(Copy)]
struct HandleScope([*mut u8; 3]);
// NOTE: Not too sure about this...
// struct HandleScope([*mut u8, ..3]);

pub fn with_handle_scope<T>(isolate: Isolate, closure: fn() -> T) -> T {
    let null = ptr::null_mut();
    let mut this: HandleScope = HandleScope([null, null, null]);
    unsafe { _ZN2v811HandleScopeC1EPNS_7IsolateE(&mut this, isolate) };
    let rval = closure();
    unsafe { _ZN2v811HandleScopeD1Ev(&mut this) };
    rval
}

// XXX(bnoordhuis) Deviates slightly from the C++ definition in that Int32 and
// Uint32 don't inherit from Integer.  As a result, New() and NewFromUnsigned()
// return the concrete type, not the base type (e.g. Int32 instead of Integer.)
macro_rules! integer_methods(
    ($ty:ident, $rty:ty) => (
        value_methods!($ty);

        impl $ty {
            pub fn New(isolate: Isolate, value: i32) -> Option<$ty> {
                let that = unsafe {
                    _ZN2v87Integer3NewEPNS_7IsolateEi(isolate, value)
                };
                match that {
                    Integer(that) => $ty(unsafe { mem::transmute(that) })
                }.to_option()
            }

            pub fn NewFromUnsigned(isolate: Isolate, value: u32) -> Option<$ty> {
                let that = unsafe {
                    _ZN2v87Integer15NewFromUnsignedEPNS_7IsolateEj(isolate,
                                                                   value)
                };
                match that {
                    Integer(that) => $ty(unsafe { mem::transmute(that) })
                }.to_option()
            }

            pub fn Value(&self) -> $rty {
                unsafe {
                    _ZNK2v87Integer5ValueEv(mem::transmute(*self)) as $rty
                }
            }
        }
    );
);

#[repr(C)]
#[derive(Copy)]
pub struct Integer(*mut *mut Integer);
integer_methods!(Integer, i64);

#[repr(C)]
#[derive(Copy)]
pub struct Int32(*mut *mut Int32);
integer_methods!(Int32, i32);

#[repr(C)]
#[derive(Copy)]
pub struct Uint32(*mut *mut Uint32);
integer_methods!(Uint32, u32);

#[repr(C)]
#[derive(Copy)]
pub struct Isolate(*mut Isolate);

impl Isolate {
    pub fn Dispose(&mut self) {
        unsafe { _ZN2v87Isolate7DisposeEv(*self); }
        *self = Isolate(ptr::null_mut());
    }

    pub fn Enter(&self) {
        unsafe { _ZN2v87Isolate5EnterEv(*self) }
    }

    pub fn Exit(&self) {
        unsafe { _ZN2v87Isolate4ExitEv(*self) }
    }

    pub fn New(_: Option<CreateParams>) -> Option<Isolate> {
        let params = Default::default();
        match unsafe { _ZN2v87Isolate3NewERKNS0_12CreateParamsE(&params) } {
            Isolate(that) if that.is_null() => None,
            this => Some(this),
        }
    }

    fn raw_ptr(&self) -> *mut Isolate {
        match *self { Isolate(that) => that }
    }
}

impl PartialEq for Isolate {
    fn eq(&self, that: &Isolate) -> bool {
        self.raw_ptr() == that.raw_ptr()
    }
}

impl fmt::Debug for Isolate {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Isolate({:p})", self.raw_ptr())
    }
}

// Can't use a RAII type for Isolate::Scope because of
// https://github.com/rust-lang/rust/issues/17858
pub fn with_isolate_scope<T>(isolate: Isolate, closure: fn() -> T) -> T {
    isolate.Enter();
    let rval = closure();
    isolate.Exit();
    rval
}

#[repr(C)]
#[derive(Copy)]
struct Locker([*mut u8; 3]);
// NOTE: Not too sure about this...
// struct Locker([*mut u8, ..3]);

impl Locker {
    pub fn IsActive() -> bool {
        unsafe { _ZN2v86Locker8IsActiveEv() }
    }

    pub fn IsLocked(isolate: Isolate) -> bool {
        unsafe { _ZN2v86Locker8IsLockedEPNS_7IsolateE(isolate) }
    }
}

pub fn with_locker<T>(isolate: Isolate, closure: fn() -> T) -> T {
    let null = ptr::null_mut();
    let mut this = Locker([null, null, null]);
    unsafe { _ZN2v86Locker10InitializeEPNS_7IsolateE(&mut this, isolate) };
    let rval = closure();
    unsafe { _ZN2v86LockerD1Ev(&mut this) };
    rval
}

#[repr(C)]
#[derive(Copy)]
pub struct Number(*mut *mut Number);

value_methods!(Number);

impl Number {
    pub fn New(isolate: Isolate, value: f64) -> Option<Number> {
        unsafe { _ZN2v86Number3NewEPNS_7IsolateEd(isolate, value) }.to_option()
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Object(*mut *mut Object);

value_methods!(Object);

impl Object {
    pub fn Get<K: IndexT>(&self, key: K) -> Option<Value> {
        key.get(*self).to_option()
    }

    pub fn New(isolate: Isolate) -> Option<Object> {
        unsafe { _ZN2v86Object3NewEPNS_7IsolateE(isolate) }.to_option()
    }

    pub fn Set<K: IndexT, V: ValueT>(&self, key: K, value: V) -> bool {
        key.set(*self, value.as_val())
    }
}

impl IndexT for u32 {
    fn get(&self, object: Object) -> Value {
        unsafe { _ZN2v86Object3GetEj(object, *self) }
    }

    fn set(&self, object: Object, value: Value) -> bool {
        unsafe {
            _ZN2v86Object3SetEjNS_6HandleINS_5ValueEEE(object, *self, value)
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ObjectTemplate(*mut *mut ObjectTemplate);

impl Default for ObjectTemplate {
    fn default() -> ObjectTemplate {
        ObjectTemplate(ptr::null_mut())
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Primitive(*mut *mut Primitive);

value_methods!(Primitive);

pub fn Null(isolate: Isolate) -> Primitive {
    GetRoot(Primitive, isolate, kNullValueRootIndex)
}

pub fn Undefined(isolate: Isolate) -> Primitive {
    GetRoot(Primitive, isolate, kUndefinedValueRootIndex)
}

pub fn True(isolate: Isolate) -> Boolean {
    GetRoot(Boolean, isolate, kTrueValueRootIndex)
}

pub fn False(isolate: Isolate) -> Boolean {
    GetRoot(Boolean, isolate, kFalseValueRootIndex)
}

fn GetRoot<T>(make: fn(*mut *mut T) -> T, isolate: Isolate, index: usize) -> T {
    let base = match isolate { Isolate(that) => that as usize };
    let addr = base + kIsolateRootsOffset + index * kApiPointerSize;
    make(addr as *mut *mut T)
}

#[repr(C)]
#[derive(Copy)]
pub struct ResourceConstraints {
    max_semi_space_size: i32,
    max_old_space_size: i32,
    max_executable_size: i32,
    stack_limit: *mut u32,
    max_available_threads: i32,
    code_range_size: libc::size_t,
}

impl Default for ResourceConstraints {
    fn default() -> ResourceConstraints {
        ResourceConstraints {
            max_semi_space_size: 0,
            max_old_space_size: 0,
            max_executable_size: 0,
            stack_limit: ptr::null_mut(),
            max_available_threads: 0,
            code_range_size: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ReturnValue(*mut *mut Value);

impl ReturnValue {
    pub fn GetIsolate(&self) -> Isolate {
        match *self {
            ReturnValue(this) => {
                let that = this as usize - 2 * kApiPointerSize;
                let that: *const *mut Isolate = unsafe { mem::transmute(that) };
                Isolate(unsafe { *that })
            }
        }
    }

    pub fn Set<T: ValueT>(&self, value: T) {
        self.set(value.as_val())
    }

    pub fn SetEmptyString(&self) {
        self.set(GetRoot(Value, self.GetIsolate(), kEmptyStringRootIndex))
    }

    pub fn SetNull(&self) {
        self.set(GetRoot(Value, self.GetIsolate(), kNullValueRootIndex))
    }

    pub fn SetUndefined(&self) {
        self.set(GetRoot(Value, self.GetIsolate(), kUndefinedValueRootIndex))
    }

    pub fn set(&self, value: Value) {
        match (*self, value) {
            (ReturnValue(this), Value(that)) => unsafe { *this = *that }
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Script(*mut *mut Script);

data_methods!(Script);

impl Script {
    pub fn Compile(source: String,
                   origin: Option<ScriptOrigin>) -> Option<Script> {
        let _ = origin;
        unsafe {
            _ZN2v86Script7CompileENS_6HandleINS_6StringEEEPNS_12ScriptOriginE(
                    source, ptr::null_mut())
        }.to_option()
    }

    pub fn Run(&self) -> Option<Value> {
        unsafe { _ZN2v86Script3RunEv(*self) }.to_option()
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ScriptOrigin;

#[repr(C)]
#[derive(Copy)]
pub struct Signature(*mut *mut Signature);

data_methods!(Signature);

#[repr(C)]
#[derive(Copy)]
pub struct String(*mut *mut String);

value_methods!(String);

impl String {
    pub fn Empty(isolate: Isolate) -> String {
        GetRoot(String, isolate, kEmptyStringRootIndex)
    }

    pub fn Length(&self) -> isize {
        unsafe { _ZNK2v86String6LengthEv(*self) as isize }
    }

    pub fn NewFromUtf8(isolate: Isolate, data: &str,
                       typ: NewStringType) -> Option<String> {
        unsafe {
            _ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS0_13NewStringTypeEi(
                    isolate, data.as_ptr(), typ, data.len() as i32)
        }.to_option()
    }
}

#[repr(C)]
#[derive(Debug,Copy)]
pub enum NewStringType {
    kNormalString,
    kInternalizedString,
    kUndetectableString,
}

#[repr(C)]
#[derive(Copy)]
struct Unlocker(*mut u8);

pub fn with_unlocker<T>(isolate: Isolate, closure: fn() -> T) -> T {
    let mut this = Unlocker(ptr::null_mut());
    unsafe { _ZN2v88Unlocker10InitializeEPNS_7IsolateE(&mut this, isolate) };
    let rval = closure();
    unsafe { _ZN2v88UnlockerD1Ev(&mut this) };
    rval
}

#[repr(C)]
#[derive(Copy)]
pub struct V8(*mut V8);

impl V8 {
    pub fn Initialize() -> bool {
        unsafe { _ZN2v82V810InitializeEv() }
    }

    pub fn Dispose() -> bool {
        unsafe { _ZN2v82V87DisposeEv() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Value(*mut *mut Value);

value_methods!(Value);
