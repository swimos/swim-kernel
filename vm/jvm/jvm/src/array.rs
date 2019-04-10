use core::ops::Deref;
use swim_jvm_sys::{jsize, jarray, jvalue};
use crate::error::Result;
use crate::object::JObject;
use crate::env::JEnv;
use crate::convert::{FromJava, IntoJava, TryFromJava, TryIntoJava};

#[derive(Clone, Copy)]
pub struct JArray {
    object: JObject,
}

unsafe impl Send for JArray {}
unsafe impl Sync for JArray {}

impl JArray {
    #[inline]
    pub fn get_env(&self) -> &JEnv {
        self.object.get_env()
    }

    #[inline]
    pub fn len(&self) -> jsize {
        self.get_env().get_array_length(self)
    }
}

impl Deref for JArray {
    type Target = JObject;

    #[inline]
    fn deref(&self) -> &JObject {
        &self.object
    }
}

impl FromJava<jarray> for JArray {
    #[inline]
    fn from_java(env: &JEnv, string: jarray) -> Self {
        Self { object: JObject::from_java(env, string) }
    }
}

impl TryFromJava<jarray> for JArray {
    #[inline]
    fn try_from_java(env: &JEnv, string: jarray) -> Result<Self> {
        Ok(Self::from_java(env, string))
    }
}

impl IntoJava<jarray> for JArray {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jarray {
        self.object.into()
    }
}

impl<'a> IntoJava<jarray> for &'a JArray {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jarray {
        self.object.into()
    }
}

impl TryIntoJava<jarray> for JArray {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jarray> {
        Ok(self.object.into())
    }
}

impl<'a> TryIntoJava<jarray> for &'a JArray {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jarray> {
        Ok(self.object.into())
    }
}

impl From<JObject> for JArray {
    #[inline]
    fn from(object: JObject) -> JArray {
        JArray { object: object }
    }
}

impl Into<JObject> for JArray {
    #[inline]
    fn into(self) -> JObject {
        self.object
    }
}

impl<'a> Into<JObject> for &'a JArray {
    #[inline]
    fn into(self) -> JObject {
        self.object
    }
}

impl Into<jarray> for JArray {
    #[inline]
    fn into(self) -> jarray {
        self.object.into()
    }
}

impl<'a> Into<jarray> for &'a JArray {
    #[inline]
    fn into(self) -> jarray {
        self.object.into()
    }
}

impl Into<jvalue> for JArray {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.object.into() }
    }
}

impl<'a> Into<jvalue> for &'a JArray {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.object.into() }
    }
}
