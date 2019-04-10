use core::ops::Deref;
use swim_c::cstr::CStr;
use swim_jvm_sys::{jclass, jvalue};
use crate::error::Result;
use crate::object::JObject;
use crate::method::JMethod;
use crate::env::JEnv;
use crate::convert::{FromJava, TryFromJava, IntoJava, TryIntoJava};

#[derive(Clone, Copy)]
pub struct JClass {
    object: JObject,
}

unsafe impl Send for JClass {}
unsafe impl Sync for JClass {}

impl JClass {
    #[inline]
    pub fn get_env(&self) -> &JEnv {
        self.object.get_env()
    }

    #[inline]
    pub fn get_method<S, T>(&self, name: S, sig: T) -> Result<JMethod>
            where S: AsRef<CStr>, T: AsRef<CStr> {
        self.get_env().get_method_id(self, name, sig)
    }
}

impl Deref for JClass {
    type Target = JObject;

    #[inline]
    fn deref(&self) -> &JObject {
        &self.object
    }
}

impl FromJava<jclass> for JClass {
    #[inline]
    fn from_java(env: &JEnv, cls: jclass) -> Self {
        Self { object: JObject::from_java(env, cls) }
    }
}

impl TryFromJava<jclass> for JClass {
    #[inline]
    fn try_from_java(env: &JEnv, cls: jclass) -> Result<Self> {
        Ok(Self::from_java(env, cls))
    }
}

impl IntoJava<jclass> for JClass {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jclass {
        self.object.into()
    }
}

impl<'a> IntoJava<jclass> for &'a JClass {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jclass {
        self.object.into()
    }
}

impl TryIntoJava<jclass> for JClass {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jclass> {
        Ok(self.object.into())
    }
}

impl<'a> TryIntoJava<jclass> for &'a JClass {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jclass> {
        Ok(self.object.into())
    }
}

impl From<JObject> for JClass {
    #[inline]
    fn from(object: JObject) -> JClass {
        JClass { object: object }
    }
}

impl Into<JObject> for JClass {
    #[inline]
    fn into(self) -> JObject {
        self.object
    }
}

impl<'a> Into<JObject> for &'a JClass {
    #[inline]
    fn into(self) -> JObject {
        self.object
    }
}

impl Into<jclass> for JClass {
    #[inline]
    fn into(self) -> jclass {
        self.object.into()
    }
}

impl<'a> Into<jclass> for &'a JClass {
    #[inline]
    fn into(self) -> jclass {
        self.object.into()
    }
}

impl Into<jvalue> for JClass {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.object.into() }
    }
}

impl<'a> Into<jvalue> for &'a JClass {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.object.into() }
    }
}
