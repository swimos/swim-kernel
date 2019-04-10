use core::ops::Deref;
use swim_jvm_sys::{jsize, jstring, jvalue};
use crate::error::Result;
use crate::object::JObject;
use crate::env::JEnv;
use crate::convert::{FromJava, IntoJava, TryFromJava, TryIntoJava};

#[derive(Clone, Copy)]
pub struct JString {
    object: JObject,
}

unsafe impl Send for JString {}
unsafe impl Sync for JString {}

impl JString {
    #[inline]
    pub fn get_env(&self) -> &JEnv {
        self.object.get_env()
    }

    #[inline]
    pub fn len(&self) -> jsize {
        self.get_env().get_string_length(self)
    }

    #[inline]
    pub fn len_utf(&self) -> jsize {
        self.get_env().get_string_utf_length(self)
    }
}

impl Deref for JString {
    type Target = JObject;

    #[inline]
    fn deref(&self) -> &JObject {
        &self.object
    }
}

impl FromJava<jstring> for JString {
    #[inline]
    fn from_java(env: &JEnv, string: jstring) -> Self {
        Self { object: JObject::from_java(env, string) }
    }
}

impl TryFromJava<jstring> for JString {
    #[inline]
    fn try_from_java(env: &JEnv, string: jstring) -> Result<Self> {
        Ok(Self::from_java(env, string))
    }
}

impl IntoJava<jstring> for JString {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jstring {
        self.object.into()
    }
}

impl<'a> IntoJava<jstring> for &'a JString {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jstring {
        self.object.into()
    }
}

impl TryIntoJava<jstring> for JString {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jstring> {
        Ok(self.object.into())
    }
}

impl<'a> TryIntoJava<jstring> for &'a JString {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jstring> {
        Ok(self.object.into())
    }
}

impl From<JObject> for JString {
    #[inline]
    fn from(object: JObject) -> JString {
        JString { object: object }
    }
}

impl Into<JObject> for JString {
    #[inline]
    fn into(self) -> JObject {
        self.object
    }
}

impl<'a> Into<JObject> for &'a JString {
    #[inline]
    fn into(self) -> JObject {
        self.object
    }
}

impl Into<jstring> for JString {
    #[inline]
    fn into(self) -> jstring {
        self.object.into()
    }
}

impl<'a> Into<jstring> for &'a JString {
    #[inline]
    fn into(self) -> jstring {
        self.object.into()
    }
}

impl Into<jvalue> for JString {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.object.into() }
    }
}

impl<'a> Into<jvalue> for &'a JString {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.object.into() }
    }
}
