#[macro_export]
macro_rules! jni {
    ($env:expr, $name:tt $(, $arg:expr )*) => ({
        #[allow(unused_unsafe)]
        unsafe { ((**$env).$name)($env $(, $arg)*) }
    });
}

#[macro_export]
macro_rules! jni_exception_check {
    ($env:expr) => ({
        let env: *mut $crate::JNIEnv = $env.into();
        if jni!(env, ExceptionCheck) == $crate::JNI_TRUE {
            return core::result::Result::Err(::core::convert::From::from($crate::JError::from(env)))
        }
    });
}

#[macro_export]
macro_rules! jni_exception_panic {
    ($env:expr) => (
        if jni!($env, ExceptionCheck) == $crate::JNI_TRUE {
            $crate::JError::panic($env)
        }
    );
}

#[macro_export]
macro_rules! try_jni {
    ($env:expr, $name:tt $(, $arg:expr )*) => ({
        let res = jni!($env, $name $(, $arg)*);
        jni_exception_check!($env);
        res
    });
}

#[macro_export]
macro_rules! run_jni {
    ($env:expr, $name:tt $(, $arg:expr )*) => ({
        let res = jni!($env, $name $(, $arg)*);
        jni_exception_panic!($env);
        res
    });
}

#[macro_export]
macro_rules! jni_call_method {
    ($env:expr, $invoke:ident, $obj:expr, $mid:expr $(, $arg:expr )*) => ({
        let env: *mut $crate::JNIEnv = $env.into();
        let obj: $crate::jobject = $obj.into();
        let mid: $crate::jmethodID = $mid.into();
        let args = [$($arg.into()),*];
        jni!(env, $invoke, obj, mid, args.as_ptr())
    });
}

#[macro_export]
macro_rules! jni_call_void_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallVoidMethodA, $obj, $mid $(, $arg)*);
    );
}

#[macro_export]
macro_rules! jni_call_boolean_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallBooleanMethodA, $obj, $mid $(, $arg)*) == $crate::JNI_TRUE
    );
}

#[macro_export]
macro_rules! jni_call_byte_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallByteMethodA, $obj, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_char_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallCharMethodA, $obj, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_short_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallShortMethodA, $obj, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_int_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallIntMethodA, $obj, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_long_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallLongMethodA, $obj, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_float_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallFloatMethodA, $obj, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_double_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallDoubleMethodA, $obj, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_object_method {
    ($env:expr, $obj:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_method!($env, CallObjectMethodA, $obj, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_method {
    ($env:expr, $invoke:ident, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => ({
        let env: *mut $crate::JNIEnv = $env.into();
        let obj: $crate::jobject = $obj.into();
        let cls: $crate::jclass = $cls.into();
        let mid: $crate::jmethodID = $mid.into();
        let args = [$($arg.into()),*];
        jni!(env, $invoke, obj, cls, mid, args.as_ptr())
    });
}

#[macro_export]
macro_rules! jni_call_nonvirtual_void_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualVoidMethodA, $obj, $cls, $mid $(, $arg)*);
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_boolean_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualBooleanMethodA, $obj, $cls, $mid $(, $arg)*) == $crate::JNI_TRUE
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_byte_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualByteMethodA, $obj, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_char_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualCharMethodA, $obj, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_short_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualShortMethodA, $obj, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_int_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualIntMethodA, $obj, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_long_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualLongMethodA, $obj, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_float_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualFloatMethodA, $obj, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_double_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualDoubleMethodA, $obj, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_nonvirtual_object_method {
    ($env:expr, $obj:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_nonvirtual_method!($env, CallNonvirtualObjectMethodA, $obj, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_static_method {
    ($env:expr, $invoke:ident, $cls:expr, $mid:expr $(, $arg:expr )*) => ({
        let env: *mut $crate::JNIEnv = $env.into();
        let cls: $crate::jclass = $cls.into();
        let mid: $crate::jmethodID = $mid.into();
        let args = [$($arg.into()),*];
        jni!(env, $invoke, cls, mid, args.as_ptr())
    });
}

#[macro_export]
macro_rules! jni_call_static_void_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticVoidMethodA, $cls, $mid $(, $arg)*);
    );
}

#[macro_export]
macro_rules! jni_call_static_boolean_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticBooleanMethodA, $cls, $mid $(, $arg)*) == $crate::JNI_TRUE
    );
}

#[macro_export]
macro_rules! jni_call_static_byte_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticByteMethodA, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_static_char_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticCharMethodA, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_static_short_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticShortMethodA, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_static_int_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticIntMethodA, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_static_long_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticLongMethodA, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_static_float_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticFloatMethodA, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_static_double_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticDoubleMethodA, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_call_static_object_method {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, CallStaticObjectMethodA, $cls, $mid $(, $arg)*)
    );
}

#[macro_export]
macro_rules! jni_new_object {
    ($env:expr, $cls:expr, $mid:expr $(, $arg:expr )*) => (
        jni_call_static_method!($env, NewObjectA, $cls, $mid $(, $arg)*);
    );
}

#[macro_export]
macro_rules! extern_java {
    // pub static fn $name(env, cls, args*) $body $extern_java*
    ($(#[$attr:meta])* pub static fn $name:ident($env:ident, $cls:ident $(, $arg:ident: $type:ty )*) $body:block $(#[$attr2:meta])* pub $($rest:tt)*) => (
        extern_java!($(#[$attr])* pub static fn $name($env, $cls $(, $arg: $type)*) $body);
        extern_java!($(#[$attr2])* pub $($rest)*);
    );

    // pub static fn $name(env, cls, args*) $body
    ($(#[$attr:meta])* pub static fn $name:ident($env:ident, $cls:ident $(, $arg:ident: $type:ty )*) $body:block) => (
        #[no_mangle] $(#[$attr])* pub unsafe extern "C" fn $name(_env: *mut $crate::JNIEnv, _cls: $crate::jclass $(, $arg: $type)*) {
            #[allow(unused_variables)]
            let $env = &$crate::JEnv::new(_env);
            #[allow(unused_variables)]
            let $cls = $crate::IntoJava::<$crate::JClass>::into_java(_cls, $env);
            $body
        }
    );

    // pub static fn $name(env, cls, args*) -> $ret $body $extern_java*
    ($(#[$attr:meta])* pub static fn $name:ident($env:ident, $cls:ident $(, $arg:ident: $type:ty )*) -> $ret:ty $body:block $(#[$attr2:meta])* pub $($rest:tt)*) => (
        extern_java!($(#[$attr])* pub static fn $name($env, $cls $(, $arg: $type)*) -> $ret $body);
        extern_java!($(#[$attr2])* pub $($rest)*);
    );

    // pub static fn $name(env, cls, args*) -> $ret $body
    ($(#[$attr:meta])* pub static fn $name:ident($env:ident, $cls:ident $(, $arg:ident: $type:ty )*) -> $ret:ty $body:block) => (
        #[no_mangle] $(#[$attr])* pub unsafe extern "C" fn $name(_env: *mut $crate::JNIEnv, _cls: $crate::jclass $(, $arg: $type)*) -> $ret {
            #[allow(unused_variables)]
            let $env = &$crate::JEnv::new(_env);
            #[allow(unused_variables)]
            let $cls = $crate::IntoJava::<$crate::JClass>::into_java(_cls, $env);
            $body
        }
    );

    // pub fn $name(env, obj, args*) $body $extern_java*
    ($(#[$attr:meta])* pub fn $name:ident($env:ident, $obj:ident $(, $arg:ident: $type:ty )*) $body:block $(#[$attr2:meta])* pub $($rest:tt)*) => (
        extern_java!($(#[$attr])* pub fn $name($env, $obj $(, $arg: $type)*) $body);
        extern_java!($(#[$attr2])* pub $($rest)*);
    );

    // pub fn $name(env, obj, args*) $body
    ($(#[$attr:meta])* pub fn $name:ident($env:ident, $obj:ident $(, $arg:ident: $type:ty )*) $body:block) => (
        #[no_mangle] $(#[$attr])* pub unsafe extern "C" fn $name(_env: *mut $crate::JNIEnv, _obj: $crate::jclass $(, $arg: $type)*) {
            #[allow(unused_variables)]
            let $env = &$crate::JEnv::new(_env);
            #[allow(unused_variables)]
            let $obj = $crate::IntoJava::<$crate::JObject>::into_java(_obj, $env);
            $body
        }
    );

    // pub fn $name(env, obj, args*) -> $ret $body $extern_java*
    ($(#[$attr:meta])* pub fn $name:ident($env:ident, $obj:ident $(, $arg:ident: $type:ty )*) -> $ret:ty $body:block $(#[$attr2:meta])* pub $($rest:tt)*) => (
        extern_java!($(#[$attr])* pub fn $name($env, $obj $(, $arg: $type)*) -> $ret $body);
        extern_java!($(#[$attr2])* pub $($rest)*);
    );

    // pub fn $name(env, obj, args*) -> $ret $body
    ($(#[$attr:meta])* pub fn $name:ident($env:ident, $obj:ident $(, $arg:ident: $type:ty )*) -> $ret:ty $body:block) => (
        #[no_mangle] $(#[$attr])* pub unsafe extern "C" fn $name(_env: *mut $crate::JNIEnv, _obj: $crate::jclass $(, $arg: $type)*) -> $ret {
            #[allow(unused_variables)]
            let $env = &$crate::JEnv::new(_env);
            #[allow(unused_variables)]
            let $obj = $crate::IntoJava::<$crate::JObject>::into_java(_obj, $env);
            $body
        }
    );
}
