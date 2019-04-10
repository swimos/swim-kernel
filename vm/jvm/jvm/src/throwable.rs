use core::ops::Deref;
use swim_jvm_sys::{jthrowable, jvalue};
use crate::error::Result;
use crate::object::JObject;
use crate::env::JEnv;
use crate::convert::{FromJava, IntoJava, TryFromJava, TryIntoJava};

#[derive(Clone, Copy, Debug)]
pub struct JThrowable {
    object: JObject,
}

unsafe impl Send for JThrowable {}
unsafe impl Sync for JThrowable {}

impl JThrowable {
    #[inline]
    pub fn get_env(&self) -> &JEnv {
        self.object.get_env()
    }
}

impl Deref for JThrowable {
    type Target = JObject;

    #[inline]
    fn deref(&self) -> &JObject {
        &self.object
    }
}

impl FromJava<jthrowable> for JThrowable {
    #[inline]
    fn from_java(env: &JEnv, throwable: jthrowable) -> Self {
        Self { object: JObject::from_java(env, throwable) }
    }
}

impl TryFromJava<jthrowable> for JThrowable {
    #[inline]
    fn try_from_java(env: &JEnv, throwable: jthrowable) -> Result<Self> {
        Ok(Self::from_java(env, throwable))
    }
}

impl IntoJava<jthrowable> for JThrowable {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jthrowable {
        self.object.into()
    }
}

impl<'a> IntoJava<jthrowable> for &'a JThrowable {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jthrowable {
        self.object.into()
    }
}

impl TryIntoJava<jthrowable> for JThrowable {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jthrowable> {
        Ok(self.object.into())
    }
}

impl<'a> TryIntoJava<jthrowable> for &'a JThrowable {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jthrowable> {
        Ok(self.object.into())
    }
}

impl From<JObject> for JThrowable {
    #[inline]
    fn from(object: JObject) -> JThrowable {
        JThrowable { object: object }
    }
}

impl Into<JObject> for JThrowable {
    #[inline]
    fn into(self) -> JObject {
        self.object
    }
}

impl<'a> Into<JObject> for &'a JThrowable {
    #[inline]
    fn into(self) -> JObject {
        self.object
    }
}

impl Into<jthrowable> for JThrowable {
    #[inline]
    fn into(self) -> jthrowable {
        self.object.into()
    }
}

impl<'a> Into<jthrowable> for &'a JThrowable {
    #[inline]
    fn into(self) -> jthrowable {
        self.object.into()
    }
}

impl Into<jvalue> for JThrowable {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.object.into() }
    }
}

impl<'a> Into<jvalue> for &'a JThrowable {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.object.into() }
    }
}
