#![feature(plugin, negate_unsigned, convert)]
#![plugin(regex_macros)]
extern crate regex;
extern crate v8;

#[test]
fn up_and_down() {
    with_isolate_and_context(move |_, _| {});
    with_isolate_and_context(move |_, _| {});
}

#[test]
fn lock_and_unlock() {
    with_isolate_and_context(move |isolate, _| {
        assert!(v8::Locker::IsActive());
        assert!(v8::Locker::IsLocked(isolate));
        v8::with_unlocker(isolate, move || {
            assert!(!v8::Locker::IsLocked(isolate));
            v8::with_locker(isolate, move || {
                assert!(v8::Locker::IsLocked(isolate));
            });
            assert!(!v8::Locker::IsLocked(isolate));
        });
        assert!(v8::Locker::IsLocked(isolate));
    });
}

#[test]
fn eq() {
    with_isolate_and_context(move |isolate, _| {
        let a = v8::Object::New(isolate).unwrap();
        let b = v8::Object::New(isolate).unwrap();
        assert_eq!(a, a);
        assert_eq!(b, b);
        assert!(a != b);
    });
}

#[test]
fn show() {
    with_isolate_and_context(move |isolate, _| {
        let val = v8::Number::New(isolate, 13.37).unwrap();
        let re = regex!("Number\\(0x[0-9a-f]+\\)");
        let pp = format!("{}", val);
        assert!(re.is_match(pp.as_str()));
        let val = v8::Object::New(isolate).unwrap();
        let re = regex!("Object\\(0x[0-9a-f]+\\)");
        let pp = format!("{}", val);
        assert!(re.is_match(pp.as_str()));
    });
}

#[test]
fn fortytwo() {
    with_isolate_and_context(move |isolate, _| {
        let source = v8::String::NewFromUtf8(isolate, "42",
                                             v8::NewStringType::kNormalString).unwrap();
        assert!(source.IsString());
        assert!(!source.IsStringObject());
        let script = v8::Script::Compile(source, None).unwrap();
        let result = script.Run().unwrap();
        assert!(!result.IsNull());
        assert!(result.IsNumber());
        assert!(!result.IsUndefined());
    });
}

#[test]
fn primitives() {
    with_isolate_and_context(move |isolate, _| {
        assert!(v8::Null(isolate).IsNull());
        assert!(v8::Undefined(isolate).IsUndefined());
        assert!(v8::True(isolate).IsTrue());
        assert!(v8::False(isolate).IsFalse());
        assert!(v8::String::Empty(isolate).IsString());
        assert!(v8::String::Empty(isolate).Length() == 0);
    });
}

#[test]
fn booleans() {
    with_isolate_and_context(move |isolate, _| {
        assert!(v8::Boolean::New(isolate, true).unwrap().IsTrue());
        assert!(v8::Boolean::New(isolate, false).unwrap().IsFalse());
        assert!(v8::Boolean::New(isolate, true).unwrap().Value());
        assert!(!v8::Boolean::New(isolate, false).unwrap().Value());
    });
}

#[test]
fn integers() {
    with_isolate_and_context(move |isolate, _| {
        assert_eq!(42, v8::Integer::New(isolate, 42).unwrap().Value());
        assert_eq!(-1 as u32 as i64,
                   v8::Integer::NewFromUnsigned(isolate, -1).unwrap().Value());
        assert_eq!(-1, v8::Int32::New(isolate, -1).unwrap().Value());
        assert_eq!(-1 as u32, v8::Uint32::New(isolate, -1).unwrap().Value());
    });
}

#[test]
fn basic_number() {
    with_isolate_and_context(move |isolate, _| {
        let val = 13.37;
        let num = v8::Number::New(isolate, val).unwrap();
        assert_eq!(val, num.NumberValue());
    });
}

#[test]
fn basic_object() {
    with_isolate_and_context(move |isolate, _| {
        let object = v8::Object::New(isolate).unwrap();
        assert!(object.IsObject());
        let key = v8::String::NewFromUtf8(isolate, "the_key",
                                          v8::NewStringType::kNormalString).unwrap();
        let value = v8::String::NewFromUtf8(isolate, "the_value",
                                            v8::NewStringType::kNormalString).unwrap();
        assert!(object.Set(key, value));
        assert!(object.Get(key).unwrap().IsString());
    });
}

#[test]
fn object_get_and_set() {
    with_isolate_and_context(move |isolate, _| {
        let obj = v8::Object::New(isolate).unwrap();
        let idx = 123 as u32;
        let key = v8::String::NewFromUtf8(isolate, "the_key",
                                          v8::NewStringType::kNormalString).unwrap();
        let abc = v8::Object::New(isolate).unwrap();
        let def = v8::Object::New(isolate).unwrap();
        assert!(obj.Set(idx, abc));
        assert!(obj.Set(key, def));
        assert_eq!(obj.Get(idx).unwrap(), abc.As());
        assert_eq!(obj.Get(key).unwrap(), def.As());
    });
}

#[test]
fn global_object() {
    with_isolate_and_context(move |isolate, context| {
        let key = v8::String::NewFromUtf8(isolate, "Object",
                                          v8::NewStringType::kNormalString).unwrap();
        assert!(context.Global().unwrap().Get(key).unwrap().IsObject());
    });
}

#[test]
fn native_api_call() {
    with_isolate_and_context(move |isolate, context| {
        extern fn f(info: v8::FunctionCallbackInfo) {
            let isolate = info.GetIsolate();
            let mut return_value = info.GetReturnValue();
            assert_eq!(isolate, return_value.GetIsolate());
            assert!(info.At(0).IsNumber());
            assert!(info.At(1).IsNumber());
            let a = info.At(0).NumberValue();
            let b = info.At(1).NumberValue();
            let x = v8::Number::New(isolate, a * b).unwrap();
            return_value.Set(x);
        }
        let t = v8::FunctionTemplate::New(isolate, Some(f),
                                          None, None, 2).unwrap();
        let key = v8::String::NewFromUtf8(isolate, "f",
                                          v8::NewStringType::kNormalString).unwrap();
        let global = context.Global().unwrap();
        global.Set(key, t.GetFunction().unwrap());
        assert!(global.Get(key).unwrap().IsFunction());
        let result = eval(isolate, "f(1337, 42)").unwrap();
        assert!(result.IsNumber());
        assert_eq!(result.NumberValue(), 1337. * 42.);
    });
}

#[test]
fn return_values() {
    with_isolate_and_context(move |isolate, context| {
        extern fn f(info: v8::FunctionCallbackInfo) {
            match info.At(0).NumberValue() {
                0. => info.GetReturnValue().SetEmptyString(),
                1. => info.GetReturnValue().SetNull(),
                _ => info.GetReturnValue().SetUndefined(),
            }
        }
        let t = v8::FunctionTemplate::New(isolate, Some(f),
                                          None, None, 1).unwrap();
        let key = v8::String::NewFromUtf8(isolate, "f",
                                          v8::NewStringType::kNormalString).unwrap();
        context.Global().unwrap().Set(key, t.GetFunction().unwrap());
        assert!(eval(isolate, "f(0)").unwrap().IsString());
        assert!(eval(isolate, "f(1)").unwrap().IsNull());
        assert!(eval(isolate, "f(2)").unwrap().IsUndefined());
    });
}

#[test]
fn function_call() {
    with_isolate_and_context(move |isolate, _| {
        let val = eval(isolate, "(function(x, y) { return x * y })").unwrap();
        let fun: v8::Function = val.As();
        let argv = [v8::Int32::New(isolate, 42).unwrap().As(),
                    v8::Int32::New(isolate, 1337).unwrap().As()];
        let result = fun.Call(v8::Null(isolate), &argv).unwrap();
        assert!(result.IsNumber());
        assert!(result.NumberValue() == 42. * 1337.);
        assert!(result.IntegerValue() == 42 * 1337);
        assert!(result.IsUint32());
        assert!(result.Uint32Value() == 42 * 1337);
        assert!(result.IsInt32());
        assert!(result.Int32Value() == 42 * 1337);
    });
}

fn eval(isolate: v8::Isolate, raw_source: &str) -> Option<v8::Value> {
    let source = v8::String::NewFromUtf8(isolate, raw_source,
                                         v8::NewStringType::kNormalString).unwrap();
    let script = v8::Script::Compile(source, None).unwrap();
    script.Run()
}

fn with_isolate_and_context<F>(closure: F) where F: Fn(v8::Isolate, v8::Context) {
    assert!(v8::V8::Initialize());
    {
        let mut isolate = v8::Isolate::New(None).unwrap();
        v8::with_locker(isolate, move || {
            v8::with_handle_scope(isolate, move || {
                v8::with_isolate_scope(isolate, move || {
                    let context = v8::Context::New(isolate).unwrap();
                    v8::with_context_scope(context, move || {
                        closure(isolate, context)
                    });
                });
            });
        });
        isolate.Dispose();
    }
    // Don't call v8::V8::Dispose(), it permanently tears down V8.
}
