use core::ptr;
use core::slice;
use swim_c::{void, cchar};
use swim_c::cstr::CStr;
use swim_jvm_sys::{JNIEnv, JNINativeMethod, JNI_TRUE};
use swim_jvm_sys::{jboolean, jbyte, jchar, jshort, jint, jlong, jfloat, jdouble, jsize};
use swim_jvm_sys::{jbooleanArray, jbyteArray, jcharArray, jshortArray};
use swim_jvm_sys::{jintArray, jlongArray, jfloatArray, jdoubleArray};
use swim_jvm_sys::{jobject, jobjectArray, jarray, jstring, jthrowable, jvalue};
use swim_jvm_sys::{jclass, jmethodID, jfieldID, jobjectRefType};
use crate::error::{Result, JError};
use crate::refs::{JGlobalRef, JLocalRef};
use crate::types::JType;
use crate::value::JValue;
use crate::object::JObject;
use crate::array::JArray;
use crate::string::JString;
use crate::throwable::JThrowable;
use crate::class::JClass;
use crate::method::JMethod;
use crate::field::JField;
use crate::convert::{IntoJava, TryIntoJava};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct JEnv {
    env: *mut JNIEnv,
}

impl JEnv {
    pub fn new(env: *mut JNIEnv) -> Self {
        Self { env: env }
    }

    pub fn get_version(&self) -> jint {
        jni!(self.env, GetVersion)
    }

    pub fn define_class<S, O>(&self, name: S, loader: O, buf: &[u8]) -> Result<JClass>
            where S: AsRef<CStr>, O: Into<jobject> {
        let name = name.as_ref();
        let loader = loader.into();
        let cls = try_jni!(self.env, DefineClass, name.as_cptr(), loader,
                           buf.as_ptr() as *const jbyte, buf.len() as jsize);
        Ok(cls.into_java(self))
    }

    pub fn find_class<S>(&self, name: S) -> Result<JClass>
            where S: AsRef<CStr> {
        let name = name.as_ref();
        let cls = try_jni!(self.env, FindClass, name.as_cptr());
        Ok(cls.into_java(self))
    }

    pub fn get_superclass<T>(&self, class: T) -> Result<JClass>
            where T: TryIntoJava<jclass> {
        class.try_into_java(self).and_then(|cls| {
            let supercls = try_jni!(self.env, GetSuperclass, cls);
            Ok(supercls.into_java(self))
        })
    }

    pub fn is_assignable_from<T, U>(&self, sub: T, sup: U) -> Result<bool>
            where T: TryIntoJava<jclass>, U: TryIntoJava<jclass> {
        let sub = sub.try_into_java(self)?;
        let sup = sup.try_into_java(self)?;
        let res = try_jni!(self.env, IsAssignableFrom, sub, sup);
        Ok(res == JNI_TRUE)
    }

    pub fn throw<T>(&self, throwable: T)
            where T: Into<jthrowable> {
        let throwable = throwable.into();
        jni!(self.env, Throw, throwable);
    }

    pub fn throw_new<C, S>(&self, class: C, msg: S)
            where C: IntoJava<jclass>, S: AsRef<CStr> {
        let class = class.into_java(self);
        let msg = msg.as_ref();
        jni!(self.env, ThrowNew, class, msg.as_cptr());
    }

    pub fn exception_occurred(&self) -> Option<JThrowable> {
        let throwable = jni!(self.env, ExceptionOccurred);
        Some(throwable.into_java(self))
    }

    pub fn exception_describe(&self) {
        jni!(self.env, ExceptionDescribe);
    }

    pub fn exception_clear(&self) {
        jni!(self.env, ExceptionClear);
    }

    pub fn exception_check(&self) -> bool {
        jni!(self.env, ExceptionCheck) == JNI_TRUE
    }

    pub fn fatal_error<S>(&self, msg: S) -> ! where S: AsRef<CStr> {
        let msg = msg.as_ref();
        jni!(self.env, FatalError, msg.as_cptr());
    }

    pub fn new_global_ref<O>(&self, obj: O) -> Result<JGlobalRef> where O: Into<jobject> {
        let obj = obj.into();
        let gref = jni!(self.env, NewGlobalRef, obj);
        if gref.is_null() {
            return Err(JError::Internal(cstr!("out of memory")));
        }
        unsafe { Ok(JGlobalRef::new(self, gref)) }
    }

    pub unsafe fn delete_global_ref<R>(&self, gref: R) where R: Into<jobject> {
        let gref = gref.into();
        jni!(self.env, DeleteGlobalRef, gref);
    }

    pub fn new_local_ref<O>(&self, obj: O) -> Result<JLocalRef> where O: Into<jobject> {
        let obj = obj.into();
        let lref = jni!(self.env, NewLocalRef, obj);
        if lref.is_null() {
            return Err(JError::Internal(cstr!("out of memory")));
        }
        unsafe { Ok(JLocalRef::new(self, lref)) }
    }

    pub unsafe fn delete_local_ref<R>(&self, lref: R) where R: Into<jobject> {
        let lref = lref.into();
        jni!(self.env, DeleteLocalRef, lref);
    }

    pub fn ensure_local_capacity(&self, capacity: jint) -> Result<()> {
        try_jni!(self.env, EnsureLocalCapacity, capacity);
        Ok(())
    }

    pub fn is_same_object<O1, O2>(&self, obj1: O1, obj2: O2) -> bool
            where O1: Into<jobject>, O2: Into<jobject> {
        let obj1 = obj1.into();
        let obj2 = obj2.into();
        jni!(self.env, IsSameObject, obj1, obj2) == JNI_TRUE
    }

    pub fn alloc_object<C>(&self, cls: C) -> Result<JObject>
            where C: TryIntoJava<jclass> {
        let cls = cls.try_into_java(self)?;
        let obj = try_jni!(self.env, AllocObject, cls);
        Ok(obj.into_java(self))
    }

    pub fn get_object_class<O>(&self, obj: O) -> Result<JClass>
            where O: Into<jobject> {
        let obj = obj.into();
        let cls = try_jni!(self.env, GetObjectClass, obj);
        Ok(cls.into_java(self))
    }

    pub fn is_instance_of<O, C>(&self, obj: O, cls: C) -> bool
            where O: Into<jobject>, C: Into<jclass> {
        let obj = obj.into();
        let cls = cls.into();
        let res = jni!(self.env, IsInstanceOf, obj, cls);
        res == JNI_TRUE
    }

    pub fn get_method_id<C, S, T>(&self, cls: C, name: S, sig: T) -> Result<JMethod>
            where C: TryIntoJava<jclass>, S: AsRef<CStr>, T: AsRef<CStr> {
        let cls = cls.try_into_java(self)?;
        let name = name.as_ref();
        let sig = sig.as_ref();
        let mid = try_jni!(self.env, GetMethodID, cls, name.as_cptr(), sig.as_cptr());
        Ok(mid.into_java(self))
    }

    pub fn get_static_method_id<C, S, T>(&self, cls: C, name: S, sig: T) -> Result<JMethod>
            where C: TryIntoJava<jclass>, S: AsRef<CStr>, T: AsRef<CStr> {
        let cls = cls.try_into_java(self)?;
        let name = name.as_ref();
        let sig = sig.as_ref();
        let mid = try_jni!(self.env, GetStaticMethodID, cls, name.as_cptr(), sig.as_cptr());
        Ok(mid.into_java(self))
    }

    pub fn call_method<O, M, R>(&self, obj: O, mid: M, ret: R, args: &[JValue]) -> Result<JValue>
            where O: Into<jobject>, M: Into<jmethodID>, R: AsRef<JType> {
        let obj = obj.into();
        let mid = mid.into();
        let ret = ret.as_ref();
        let mut jargs = [jvalue::default(); 24];
        debug_assert!(args.len() <= jargs.len());
        for i in 0..args.len() {
            jargs[i] = args[i].into()
        }
        match *ret {
            JType::Void => {
                try_jni!(self.env, CallVoidMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Void)
            },
            JType::Boolean => {
                let res = try_jni!(self.env, CallBooleanMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Boolean(res))
            },
            JType::Byte => {
                let res = try_jni!(self.env, CallByteMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Byte(res))
            },
            JType::Char => {
                let res = try_jni!(self.env, CallCharMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Char(res))
            },
            JType::Short => {
                let res = try_jni!(self.env, CallShortMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Short(res))
            },
            JType::Int => {
                let res = try_jni!(self.env, CallIntMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Int(res))
            },
            JType::Long => {
                let res = try_jni!(self.env, CallLongMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Long(res))
            },
            JType::Float => {
                let res = try_jni!(self.env, CallFloatMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Float(res))
            },
            JType::Double => {
                let res = try_jni!(self.env, CallDoubleMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Double(res))
            },
            _ => {
                let res = try_jni!(self.env, CallObjectMethodA, obj, mid, jargs.as_ptr());
                Ok(JValue::Object(res.into_java(self)))
            },
        }
    }

    pub fn call_nonvirtual_method<O, C, M, R>(&self, obj: O, cls: C, mid: M, ret: R, args: &[JValue]) -> Result<JValue>
            where O: Into<jobject>, C: Into<jclass>, M: Into<jmethodID>, R: AsRef<JType> {
        let obj = obj.into();
        let cls = cls.into();
        let mid = mid.into();
        let ret = ret.as_ref();
        let mut jargs = [jvalue::default(); 24];
        debug_assert!(args.len() <= jargs.len());
        for i in 0..args.len() {
            jargs[i] = args[i].into()
        }
        match *ret {
            JType::Void => {
                try_jni!(self.env, CallNonvirtualVoidMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Void)
            },
            JType::Boolean => {
                let res = try_jni!(self.env, CallNonvirtualBooleanMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Boolean(res))
            },
            JType::Byte => {
                let res = try_jni!(self.env, CallNonvirtualByteMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Byte(res))
            },
            JType::Char => {
                let res = try_jni!(self.env, CallNonvirtualCharMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Char(res))
            },
            JType::Short => {
                let res = try_jni!(self.env, CallNonvirtualShortMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Short(res))
            },
            JType::Int => {
                let res = try_jni!(self.env, CallNonvirtualIntMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Int(res))
            },
            JType::Long => {
                let res = try_jni!(self.env, CallNonvirtualLongMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Long(res))
            },
            JType::Float => {
                let res = try_jni!(self.env, CallNonvirtualFloatMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Float(res))
            },
            JType::Double => {
                let res = try_jni!(self.env, CallNonvirtualDoubleMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Double(res))
            },
            _ => {
                let res = try_jni!(self.env, CallNonvirtualObjectMethodA, obj, cls, mid, jargs.as_ptr());
                Ok(JValue::Object(res.into_java(self)))
            },
        }
    }

    pub fn call_static_method<C, M, R>(&self, cls: C, mid: M, ret: R, args: &[JValue]) -> Result<JValue>
            where C: Into<jclass>, M: Into<jmethodID>, R: AsRef<JType> {
        let cls = cls.into();
        let mid = mid.into();
        let ret = ret.as_ref();
        let mut jargs = [jvalue::default(); 24];
        debug_assert!(args.len() <= jargs.len());
        for i in 0..args.len() {
            jargs[i] = args[i].into()
        }
        match *ret {
            JType::Void => {
                try_jni!(self.env, CallStaticVoidMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Void)
            },
            JType::Boolean => {
                let res = try_jni!(self.env, CallStaticBooleanMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Boolean(res))
            },
            JType::Byte => {
                let res = try_jni!(self.env, CallStaticByteMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Byte(res))
            },
            JType::Char => {
                let res = try_jni!(self.env, CallStaticCharMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Char(res))
            },
            JType::Short => {
                let res = try_jni!(self.env, CallStaticShortMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Short(res))
            },
            JType::Int => {
                let res = try_jni!(self.env, CallStaticIntMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Int(res))
            },
            JType::Long => {
                let res = try_jni!(self.env, CallStaticLongMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Long(res))
            },
            JType::Float => {
                let res = try_jni!(self.env, CallStaticFloatMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Float(res))
            },
            JType::Double => {
                let res = try_jni!(self.env, CallStaticDoubleMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Double(res))
            },
            _ => {
                let res = try_jni!(self.env, CallStaticObjectMethodA, cls, mid, jargs.as_ptr());
                Ok(JValue::Object(res.into_java(self)))
            },
        }
    }

    pub fn new_object<C, M>(&self, cls: C, mid: M, args: &[JValue]) -> Result<JObject>
            where C: Into<jclass>, M: Into<jmethodID> {
        let cls = cls.into();
        let mid = mid.into();
        let mut jargs = [jvalue::default(); 24];
        debug_assert!(args.len() <= jargs.len());
        for i in 0..args.len() {
            jargs[i] = args[i].into()
        }
        let res = try_jni!(self.env, NewObjectA, cls, mid, jargs.as_ptr());
        Ok(res.into_java(self))
    }

    pub fn get_field_id<C, S, T>(&self, cls: C, name: S, sig: T) -> Result<JField>
            where C: TryIntoJava<jclass>, S: AsRef<CStr>, T: AsRef<CStr> {
        let cls = cls.try_into_java(self)?;
        let name = name.as_ref();
        let sig = sig.as_ref();
        let fid = try_jni!(self.env, GetFieldID, cls, name.as_cptr(), sig.as_cptr());
        Ok(fid.into_java(self))
    }

    pub fn get_field<O, F, T>(&self, obj: O, fid: F, ret: T) -> JValue
            where O: Into<jobject>, F: Into<jfieldID>, T: AsRef<JType> {
        let obj = obj.into();
        let fid = fid.into();
        let ret = ret.as_ref();
        match *ret {
            JType::Void => unreachable!(),
            JType::Boolean => {
                let val = jni!(self.env, GetBooleanField, obj, fid);
                JValue::Boolean(val)
            },
            JType::Byte => {
                let val = jni!(self.env, GetByteField, obj, fid);
                JValue::Byte(val)
            },
            JType::Char => {
                let val = jni!(self.env, GetCharField, obj, fid);
                JValue::Char(val)
            },
            JType::Short => {
                let val = jni!(self.env, GetShortField, obj, fid);
                JValue::Short(val)
            },
            JType::Int => {
                let val = jni!(self.env, GetIntField, obj, fid);
                JValue::Int(val)
            },
            JType::Long => {
                let val = jni!(self.env, GetLongField, obj, fid);
                JValue::Long(val)
            },
            JType::Float => {
                let val = jni!(self.env, GetFloatField, obj, fid);
                JValue::Float(val)
            },
            JType::Double => {
                let val = jni!(self.env, GetDoubleField, obj, fid);
                JValue::Double(val)
            },
            _ => {
                let val = jni!(self.env, GetObjectField, obj, fid);
                JValue::Object(val.into_java(self))
            },
        }
    }

    pub fn get_boolean_field<O, F>(&self, obj: O, fid: F) -> bool
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        let val = jni!(self.env, GetBooleanField, obj, fid);
        val == JNI_TRUE
    }

    pub fn get_byte_field<O, F>(&self, obj: O, fid: F) -> jbyte
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        jni!(self.env, GetByteField, obj, fid)
    }

    pub fn get_char_field<O, F>(&self, obj: O, fid: F) -> jchar
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        jni!(self.env, GetCharField, obj, fid)
    }

    pub fn get_short_field<O, F>(&self, obj: O, fid: F) -> jshort
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        jni!(self.env, GetShortField, obj, fid)
    }

    pub fn get_int_field<O, F>(&self, obj: O, fid: F) -> jint
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        jni!(self.env, GetIntField, obj, fid)
    }

    pub fn get_long_field<O, F>(&self, obj: O, fid: F) -> jlong
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        jni!(self.env, GetLongField, obj, fid)
    }

    pub fn get_float_field<O, F>(&self, obj: O, fid: F) -> jfloat
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        jni!(self.env, GetFloatField, obj, fid)
    }

    pub fn get_double_field<O, F>(&self, obj: O, fid: F) -> jdouble
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        jni!(self.env, GetDoubleField, obj, fid)
    }

    pub fn get_object_field<O, F>(&self, obj: O, fid: F) -> JObject
            where O: Into<jobject>, F: Into<jfieldID> {
        let obj = obj.into();
        let fid = fid.into();
        let val = jni!(self.env, GetObjectField, obj, fid);
        val.into_java(self)
    }

    pub fn set_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: IntoJava<JValue> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into_java(self);
        match val {
            JValue::Void => unreachable!(),
            JValue::Boolean(val) => {
                jni!(self.env, SetBooleanField, obj, fid, val);
            },
            JValue::Byte(val) => {
                jni!(self.env, SetByteField, obj, fid, val);
            },
            JValue::Char(val) => {
                jni!(self.env, SetCharField, obj, fid, val);
            },
            JValue::Short(val) => {
                jni!(self.env, SetShortField, obj, fid, val);
            },
            JValue::Int(val) => {
                jni!(self.env, SetIntField, obj, fid, val);
            },
            JValue::Long(val) => {
                jni!(self.env, SetLongField, obj, fid, val);
            },
            JValue::Float(val) => {
                jni!(self.env, SetFloatField, obj, fid, val);
            },
            JValue::Double(val) => {
                jni!(self.env, SetDoubleField, obj, fid, val);
            },
            JValue::Object(val) => {
                jni!(self.env, SetObjectField, obj, fid, val.into());
            },
        };
        jni_exception_panic!(self.env);
    }

    pub fn set_boolean_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<bool> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetBooleanField, obj, fid, val as jboolean);
    }

    pub fn set_byte_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<jbyte> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetByteField, obj, fid, val);
    }

    pub fn set_char_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<jchar> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetCharField, obj, fid, val);
    }

    pub fn set_short_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<jshort> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetShortField, obj, fid, val);
    }

    pub fn set_int_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<jint> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetIntField, obj, fid, val);
    }

    pub fn set_long_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<jlong> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetLongField, obj, fid, val);
    }

    pub fn set_float_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<jfloat> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetFloatField, obj, fid, val);
    }

    pub fn set_double_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<jdouble> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetDoubleField, obj, fid, val);
    }

    pub fn set_object_field<O, F, V>(&self, obj: O, fid: F, val: V)
            where O: Into<jobject>, F: Into<jfieldID>, V: Into<jobject> {
        let obj = obj.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetObjectField, obj, fid, val);
    }

    pub fn get_static_field_id<C, S, T>(&self, cls: C, name: S, sig: T) -> Result<JField>
            where C: TryIntoJava<jclass>, S: AsRef<CStr>, T: AsRef<CStr> {
        let cls = cls.try_into_java(self)?;
        let name = name.as_ref();
        let sig = sig.as_ref();
        let fid = try_jni!(self.env, GetStaticFieldID, cls, name.as_cptr(), sig.as_cptr());
        Ok(fid.into_java(self))
    }

    pub fn get_static_field<C, F, T>(&self, cls: C, fid: F, ret: T) -> JValue
            where C: Into<jclass>, F: Into<jfieldID>, T: AsRef<JType> {
        let cls = cls.into();
        let fid = fid.into();
        let ret = ret.as_ref();
        match *ret {
            JType::Void => unreachable!(),
            JType::Boolean => {
                let val = jni!(self.env, GetStaticBooleanField, cls, fid);
                JValue::Boolean(val)
            },
            JType::Byte => {
                let val = jni!(self.env, GetStaticByteField, cls, fid);
                JValue::Byte(val)
            },
            JType::Char => {
                let val = jni!(self.env, GetStaticCharField, cls, fid);
                JValue::Char(val)
            },
            JType::Short => {
                let val = jni!(self.env, GetStaticShortField, cls, fid);
                JValue::Short(val)
            },
            JType::Int => {
                let val = jni!(self.env, GetStaticIntField, cls, fid);
                JValue::Int(val)
            },
            JType::Long => {
                let val = jni!(self.env, GetStaticLongField, cls, fid);
                JValue::Long(val)
            },
            JType::Float => {
                let val = jni!(self.env, GetStaticFloatField, cls, fid);
                JValue::Float(val)
            },
            JType::Double => {
                let val = jni!(self.env, GetStaticDoubleField, cls, fid);
                JValue::Double(val)
            },
            _ => {
                let val = jni!(self.env, GetStaticObjectField, cls, fid);
                JValue::Object(val.into_java(self))
            },
        }
    }

    pub fn get_static_boolean_field<C, F>(&self, cls: C, fid: F) -> bool
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        let val = jni!(self.env, GetStaticBooleanField, cls, fid);
        val == JNI_TRUE
    }

    pub fn get_static_byte_field<C, F>(&self, cls: C, fid: F) -> jbyte
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        jni!(self.env, GetStaticByteField, cls, fid)
    }

    pub fn get_static_char_field<C, F>(&self, cls: C, fid: F) -> jchar
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        jni!(self.env, GetStaticCharField, cls, fid)
    }

    pub fn get_static_short_field<C, F>(&self, cls: C, fid: F) -> jshort
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        jni!(self.env, GetStaticShortField, cls, fid)
    }

    pub fn get_static_int_field<C, F>(&self, cls: C, fid: F) -> jint
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        jni!(self.env, GetStaticIntField, cls, fid)
    }

    pub fn get_static_long_field<C, F>(&self, cls: C, fid: F) -> jlong
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        jni!(self.env, GetStaticLongField, cls, fid)
    }

    pub fn get_static_float_field<C, F>(&self, cls: C, fid: F) -> jfloat
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        jni!(self.env, GetStaticFloatField, cls, fid)
    }

    pub fn get_static_double_field<C, F>(&self, cls: C, fid: F) -> jdouble
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        jni!(self.env, GetStaticDoubleField, cls, fid)
    }

    pub fn get_static_object_field<C, F>(&self, cls: C, fid: F) -> JObject
            where C: Into<jclass>, F: Into<jfieldID> {
        let cls = cls.into();
        let fid = fid.into();
        let val = jni!(self.env, GetStaticObjectField, cls, fid);
        val.into_java(self)
    }

    pub fn set_static_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: IntoJava<JValue> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into_java(self);
        match val {
            JValue::Void => unreachable!(),
            JValue::Boolean(val) => {
                jni!(self.env, SetStaticBooleanField, cls, fid, val);
            },
            JValue::Byte(val) => {
                jni!(self.env, SetStaticByteField, cls, fid, val);
            },
            JValue::Char(val) => {
                jni!(self.env, SetStaticCharField, cls, fid, val);
            },
            JValue::Short(val) => {
                jni!(self.env, SetStaticShortField, cls, fid, val);
            },
            JValue::Int(val) => {
                jni!(self.env, SetStaticIntField, cls, fid, val);
            },
            JValue::Long(val) => {
                jni!(self.env, SetStaticLongField, cls, fid, val);
            },
            JValue::Float(val) => {
                jni!(self.env, SetStaticFloatField, cls, fid, val);
            },
            JValue::Double(val) => {
                jni!(self.env, SetStaticDoubleField, cls, fid, val);
            },
            JValue::Object(val) => {
                jni!(self.env, SetStaticObjectField, cls, fid, val.into());
            },
        };
        jni_exception_panic!(self.env);
    }

    pub fn set_static_boolean_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<bool> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticBooleanField, cls, fid, val as jboolean);
    }

    pub fn set_static_byte_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<jbyte> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticByteField, cls, fid, val);
    }

    pub fn set_static_char_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<jchar> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticCharField, cls, fid, val);
    }

    pub fn set_static_short_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<jshort> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticShortField, cls, fid, val);
    }

    pub fn set_static_int_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<jint> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticIntField, cls, fid, val);
    }

    pub fn set_static_long_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<jlong> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticLongField, cls, fid, val);
    }

    pub fn set_static_float_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<jfloat> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticFloatField, cls, fid, val);
    }

    pub fn set_static_double_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<jdouble> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticDoubleField, cls, fid, val);
    }

    pub fn set_static_object_field<C, F, V>(&self, cls: C, fid: F, val: V)
            where C: Into<jclass>, F: Into<jfieldID>, V: Into<jobject> {
        let cls = cls.into();
        let fid = fid.into();
        let val = val.into();
        jni!(self.env, SetStaticObjectField, cls, fid, val);
    }

    pub fn new_string_utf<'b, S>(&self, utf: S) -> Result<JString>
            where S: Into<&'b CStr> {
        let utf = utf.into();
        let str = try_jni!(self.env, NewStringUTF, utf.as_cptr());
        Ok(str.into_java(self))
    }

    pub fn get_string_length<S>(&self, str: S) -> jsize where S: Into<jstring> {
        let str = str.into();
        jni!(self.env, GetStringLength, str)
    }

    pub fn get_string_utf_length<S>(&self, str: S) -> jsize where S: Into<jstring> {
        let str = str.into();
        jni!(self.env, GetStringUTFLength, str)
    }

    pub unsafe fn get_string_utf_chars<S>(&self, str: S, is_copy: &mut bool) -> &CStr
            where S: Into<jstring> {
        let str = str.into();
        let mut copy = (*is_copy) as jboolean;
        let cptr = jni!(self.env, GetStringUTFChars, str, &mut copy);
        *is_copy = copy == JNI_TRUE;
        CStr::from_cptr(cptr)
    }

    pub unsafe fn release_string_utf_chars<S>(&self, str: S, cstr: &CStr)
            where S: Into<jstring> {
        let str = str.into();
        jni!(self.env, ReleaseStringUTFChars, str, cstr.as_cptr());
    }

    pub fn get_string_utf_region<S>(&self, str: S, start: jsize, len: jsize, buf: &mut [cchar])
            where S: Into<jstring> {
        let str = str.into();
        jni!(self.env, GetStringUTFRegion, str, start, len, buf.as_mut_ptr());
    }

    pub fn get_array_length<A>(&self, array: A) -> jsize where A: Into<jarray> {
        let array = array.into();
        jni!(self.env, GetArrayLength, array)
    }

    pub fn new_array<T>(&self, len: jsize, typ: T) -> Result<JArray>
            where T: AsRef<JType> {
        let typ = typ.as_ref();
        match *typ {
            JType::Void => unreachable!(),
            JType::Boolean => {
                let array = try_jni!(self.env, NewBooleanArray, len);
                Ok(array.into_java(self))
            },
            JType::Byte => {
                let array = try_jni!(self.env, NewByteArray, len);
                Ok(array.into_java(self))
            },
            JType::Char => {
                let array = try_jni!(self.env, NewCharArray, len);
                Ok(array.into_java(self))
            },
            JType::Short => {
                let array = try_jni!(self.env, NewShortArray, len);
                Ok(array.into_java(self))
            },
            JType::Int => {
                let array = try_jni!(self.env, NewIntArray, len);
                Ok(array.into_java(self))
            },
            JType::Long => {
                let array = try_jni!(self.env, NewLongArray, len);
                Ok(array.into_java(self))
            },
            JType::Float => {
                let array = try_jni!(self.env, NewFloatArray, len);
                Ok(array.into_java(self))
            },
            JType::Double => {
                let array = try_jni!(self.env, NewDoubleArray, len);
                Ok(array.into_java(self))
            },
            _ => {
                let cls = try_jni!(self.env, FindClass, typ.to_class_name().as_cptr());
                let array = try_jni!(self.env, NewObjectArray, len, cls, ptr::null_mut());
                Ok(array.into_java(self))
            },
        }
    }

    pub fn new_boolean_array(&self, len: jsize) -> Result<JArray> {
        let array = try_jni!(self.env, NewBooleanArray, len);
        Ok(array.into_java(self))
    }

    pub fn new_byte_array(&self, len: jsize) -> Result<JArray> {
        let array = try_jni!(self.env, NewByteArray, len);
        Ok(array.into_java(self))
    }

    pub fn new_char_array(&self, len: jsize) -> Result<JArray> {
        let array = try_jni!(self.env, NewCharArray, len);
        Ok(array.into_java(self))
    }

    pub fn new_short_array(&self, len: jsize) -> Result<JArray> {
        let array = try_jni!(self.env, NewShortArray, len);
        Ok(array.into_java(self))
    }

    pub fn new_int_array(&self, len: jsize) -> Result<JArray> {
        let array = try_jni!(self.env, NewIntArray, len);
        Ok(array.into_java(self))
    }

    pub fn new_long_array(&self, len: jsize) -> Result<JArray> {
        let array = try_jni!(self.env, NewLongArray, len);
        Ok(array.into_java(self))
    }

    pub fn new_float_array(&self, len: jsize) -> Result<JArray> {
        let array = try_jni!(self.env, NewFloatArray, len);
        Ok(array.into_java(self))
    }

    pub fn new_double_array(&self, len: jsize) -> Result<JArray> {
        let array = try_jni!(self.env, NewDoubleArray, len);
        Ok(array.into_java(self))
    }

    pub fn new_object_array<C, O>(&self, len: jsize, cls: C, init: O) -> Result<JArray>
            where C: Into<jclass>, O: Into<jobject> {
        let cls = cls.into();
        let init = init.into();
        let array = try_jni!(self.env, NewObjectArray, len, cls, init);
        Ok(array.into_java(self))
    }

    pub fn get_object_array_element<A>(&self, array: A, index: jsize) -> JObject
            where A: Into<jobjectArray> {
        let array = array.into();
        let element = jni!(self.env, GetObjectArrayElement, array, index);
        element.into_java(self)
    }

    pub fn set_object_array_element<A, O>(&self, array: A, index: jsize, val: O)
            where A: Into<jobjectArray>, O: Into<jobject> {
        let array = array.into();
        let val = val.into();
        jni!(self.env, SetObjectArrayElement, array, index, val);
    }

    pub unsafe fn get_boolean_array_elements<A>(&self, array: A, is_copy: &mut bool) -> &mut [jboolean]
            where A: Into<jbooleanArray> {
        let array = array.into();
        let len = jni!(self.env, GetArrayLength, array);
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetBooleanArrayElements, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub unsafe fn release_boolean_array_elements<A>(&self, array: A, elems: &mut [jboolean], mode: jint)
            where A: Into<jbooleanArray> {
        let array = array.into();
        jni!(self.env, ReleaseBooleanArrayElements, array, elems.as_mut_ptr(), mode);
    }

    pub unsafe fn get_byte_array_elements<A>(&self, array: A, is_copy: &mut bool) -> &mut [jbyte]
            where A: Into<jbyteArray> {
        let array = array.into();
        let len = jni!(self.env, GetArrayLength, array);
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetByteArrayElements, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub unsafe fn release_byte_array_elements<A>(&self, array: A, elems: &mut [jbyte], mode: jint)
            where A: Into<jbyteArray> {
        let array = array.into();
        jni!(self.env, ReleaseByteArrayElements, array, elems.as_mut_ptr(), mode);
    }

    pub unsafe fn get_char_array_elements<A>(&self, array: A, is_copy: &mut bool) -> &mut [jchar]
            where A: Into<jcharArray> {
        let array = array.into();
        let len = jni!(self.env, GetArrayLength, array);
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetCharArrayElements, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub unsafe fn release_char_array_elements<A>(&self, array: A, elems: &mut [jchar], mode: jint)
            where A: Into<jcharArray> {
        let array = array.into();
        jni!(self.env, ReleaseCharArrayElements, array, elems.as_mut_ptr(), mode);
    }

    pub unsafe fn get_short_array_elements<A>(&self, array: A, is_copy: &mut bool) -> &mut [jshort]
            where A: Into<jshortArray> {
        let array = array.into();
        let len = jni!(self.env, GetArrayLength, array);
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetShortArrayElements, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub unsafe fn release_short_array_elements<A>(&self, array: A, elems: &mut [jshort], mode: jint)
            where A: Into<jshortArray> {
        let array = array.into();
        jni!(self.env, ReleaseShortArrayElements, array, elems.as_mut_ptr(), mode);
    }

    pub unsafe fn get_int_array_elements<A>(&self, array: A, is_copy: &mut bool) -> &mut [jint]
            where A: Into<jintArray> {
        let array = array.into();
        let len = jni!(self.env, GetArrayLength, array);
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetIntArrayElements, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub unsafe fn release_int_array_elements<A>(&self, array: A, elems: &mut [jint], mode: jint)
            where A: Into<jintArray> {
        let array = array.into();
        jni!(self.env, ReleaseIntArrayElements, array, elems.as_mut_ptr(), mode);
    }

    pub unsafe fn get_long_array_elements<A>(&self, array: A, is_copy: &mut bool) -> &mut [jlong]
            where A: Into<jlongArray> {
        let array = array.into();
        let len = jni!(self.env, GetArrayLength, array);
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetLongArrayElements, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub unsafe fn release_long_array_elements<A>(&self, array: A, elems: &mut [jlong], mode: jint)
            where A: Into<jlongArray> {
        let array = array.into();
        jni!(self.env, ReleaseLongArrayElements, array, elems.as_mut_ptr(), mode);
    }

    pub unsafe fn get_float_array_elements<A>(&self, array: A, is_copy: &mut bool) -> &mut [jfloat]
            where A: Into<jfloatArray> {
        let array = array.into();
        let len = jni!(self.env, GetArrayLength, array);
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetFloatArrayElements, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub unsafe fn release_float_array_elements<A>(&self, array: A, elems: &mut [jfloat], mode: jint)
            where A: Into<jfloatArray> {
        let array = array.into();
        jni!(self.env, ReleaseFloatArrayElements, array, elems.as_mut_ptr(), mode);
    }

    pub unsafe fn get_double_array_elements<A>(&self, array: A, is_copy: &mut bool) -> &mut [jdouble]
            where A: Into<jdoubleArray> {
        let array = array.into();
        let len = jni!(self.env, GetArrayLength, array);
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetDoubleArrayElements, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub unsafe fn release_double_array_elements<A>(&self, array: A, elems: &mut [jdouble], mode: jint)
            where A: Into<jdoubleArray> {
        let array = array.into();
        jni!(self.env, ReleaseDoubleArrayElements, array, elems.as_mut_ptr(), mode);
    }

    pub fn get_boolean_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &mut [jboolean])
            where A: Into<jbooleanArray> {
        let array = array.into();
        jni!(self.env, GetBooleanArrayRegion, array, start, len, buf.as_mut_ptr());
    }

    pub fn set_boolean_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &[jboolean])
            where A: Into<jbooleanArray> {
        let array = array.into();
        jni!(self.env, SetBooleanArrayRegion, array, start, len, buf.as_ptr());
    }

    pub fn get_byte_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &mut [jbyte])
            where A: Into<jbyteArray> {
        let array = array.into();
        jni!(self.env, GetByteArrayRegion, array, start, len, buf.as_mut_ptr());
    }

    pub fn set_byte_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &[jbyte])
            where A: Into<jbyteArray> {
        let array = array.into();
        jni!(self.env, SetByteArrayRegion, array, start, len, buf.as_ptr());
    }

    pub fn get_char_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &mut [jchar])
            where A: Into<jcharArray> {
        let array = array.into();
        jni!(self.env, GetCharArrayRegion, array, start, len, buf.as_mut_ptr());
    }

    pub fn set_char_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &[jchar])
            where A: Into<jcharArray> {
        let array = array.into();
        jni!(self.env, SetCharArrayRegion, array, start, len, buf.as_ptr());
    }

    pub fn get_short_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &mut [jshort])
            where A: Into<jshortArray> {
        let array = array.into();
        jni!(self.env, GetShortArrayRegion, array, start, len, buf.as_mut_ptr());
    }

    pub fn set_short_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &[jshort])
            where A: Into<jshortArray> {
        let array = array.into();
        jni!(self.env, SetShortArrayRegion, array, start, len, buf.as_ptr());
    }

    pub fn get_int_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &mut [jint])
            where A: Into<jintArray> {
        let array = array.into();
        jni!(self.env, GetIntArrayRegion, array, start, len, buf.as_mut_ptr());
    }

    pub fn set_int_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &[jint])
            where A: Into<jintArray> {
        let array = array.into();
        jni!(self.env, SetIntArrayRegion, array, start, len, buf.as_ptr());
    }

    pub fn get_long_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &mut [jlong])
            where A: Into<jlongArray> {
        let array = array.into();
        jni!(self.env, GetLongArrayRegion, array, start, len, buf.as_mut_ptr());
    }

    pub fn set_long_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &[jlong])
            where A: Into<jlongArray> {
        let array = array.into();
        jni!(self.env, SetLongArrayRegion, array, start, len, buf.as_ptr());
    }

    pub fn get_float_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &mut [jfloat])
            where A: Into<jfloatArray> {
        let array = array.into();
        jni!(self.env, GetFloatArrayRegion, array, start, len, buf.as_mut_ptr());
    }

    pub fn set_float_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &[jfloat])
            where A: Into<jfloatArray> {
        let array = array.into();
        jni!(self.env, SetFloatArrayRegion, array, start, len, buf.as_ptr());
    }

    pub fn get_double_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &mut [jdouble])
            where A: Into<jdoubleArray> {
        let array = array.into();
        jni!(self.env, GetDoubleArrayRegion, array, start, len, buf.as_mut_ptr());
    }

    pub fn set_double_array_region<A>(&self, array: A, start: jsize, len: jsize, buf: &[jdouble])
            where A: Into<jdoubleArray> {
        let array = array.into();
        jni!(self.env, SetDoubleArrayRegion, array, start, len, buf.as_ptr());
    }

    pub unsafe fn register_natives<C>(&self, cls: C, methods: &[JNINativeMethod])
            where C: Into<jclass> {
        let cls = cls.into();
        jni!(self.env, RegisterNatives, cls, methods.as_ptr(), methods.len() as jint);
    }

    pub unsafe fn unregister_natives<C>(&self, cls: C) where C: Into<jclass> {
        let cls = cls.into();
        jni!(self.env, UnregisterNatives, cls);
    }

    pub unsafe fn monitor_enter<O>(&self, obj: O) where O: Into<jobject> {
        let obj = obj.into();
        jni!(self.env, MonitorEnter, obj);
    }

    pub unsafe fn monitor_exit<O>(&self, obj: O) where O: Into<jobject> {
        let obj = obj.into();
        jni!(self.env, MonitorExit, obj);
    }

    pub unsafe fn get_primitive_array_critical<A>(&self, array: A, is_copy: &mut bool) -> *mut void
            where A: Into<jarray> {
        let array = array.into();
        let mut copy = (*is_copy) as jboolean;
        let ptr = jni!(self.env, GetPrimitiveArrayCritical, array, &mut copy);
        *is_copy = copy == JNI_TRUE;
        ptr
    }

    pub unsafe fn release_primitive_array_critical<A>(&self, array: A, carray: *mut void, mode: jint)
            where A: Into<jarray> {
        let array = array.into();
        jni!(self.env, ReleasePrimitiveArrayCritical, array, carray, mode);
    }

    pub fn new_direct_byte_buffer(&self, address: *mut void, capacity: jlong) -> JObject {
        let obj = jni!(self.env, NewDirectByteBuffer, address, capacity);
        obj.into_java(self)
    }

    pub fn get_direct_buffer_address<O>(&self, buf: O) -> *mut void where O: Into<jobject> {
        let buf = buf.into();
        jni!(self.env, GetDirectBufferAddress, buf)
    }

    pub fn get_direct_buffer_capacity<O>(&self, buf: O) -> jlong where O: Into<jobject> {
        let buf = buf.into();
        jni!(self.env, GetDirectBufferCapacity, buf)
    }

    pub fn get_object_ref_type<O>(&self, buf: O) -> jobjectRefType where O: Into<jobject> {
        let buf = buf.into();
        jni!(self.env, GetObjectRefType, buf)
    }
}

unsafe impl Send for JEnv {}
unsafe impl Sync for JEnv {}

impl From<*mut JNIEnv> for JEnv {
    #[inline]
    fn from(env: *mut JNIEnv) -> JEnv {
        JEnv { env: env }
    }
}

impl Into<*mut JNIEnv> for JEnv {
    #[inline]
    fn into(self) -> *mut JNIEnv {
        self.env
    }
}

impl<'a> Into<*mut JNIEnv> for &'a JEnv {
    #[inline]
    fn into(self) -> *mut JNIEnv {
        self.env
    }
}
