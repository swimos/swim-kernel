use core::fmt;
use swim_c::cstr::CStr;
use swim_jvm_sys::{jobject, jclass, jvalue};
use crate::error::Result;
use crate::class::JClass;
use crate::method::JMethod;
use crate::env::JEnv;
use crate::convert::{FromJava, IntoJava, TryFromJava, TryIntoJava};

#[derive(Clone, Copy, Eq)]
pub struct JObject {
    env: JEnv,
    obj: jobject,
}

unsafe impl Send for JObject {}
unsafe impl Sync for JObject {}

impl JObject {
    #[inline]
    pub fn get_env(&self) -> &JEnv {
        &self.env
    }

    #[inline]
    pub fn get_class(&self) -> Result<JClass> {
        self.env.get_object_class(self)
    }

    #[inline]
    pub fn instance_of<C>(&self, cls: C) -> bool where C: Into<jclass> {
        self.env.is_instance_of(self, cls)
    }

    #[inline]
    pub fn get_method<S, T>(&self, name: S, sig: T) -> Result<JMethod>
            where S: AsRef<CStr>, T: AsRef<CStr> {
        let class = self.get_class()?;
        class.get_method(name, sig)
    }
}

impl PartialEq for JObject {
    #[inline]
    fn eq(&self, other: &JObject) -> bool {
        self.env == other.env && self.env.is_same_object(self, other)
    }
}

impl fmt::Debug for JObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}", self.obj) // TODO
    }
}

impl FromJava<jobject> for JObject {
    #[inline]
    fn from_java(env: &JEnv, obj: jobject) -> Self {
        Self {
            env: env.clone(),
            obj: obj,
        }
    }
}

impl TryFromJava<jobject> for JObject {
    #[inline]
    fn try_from_java(env: &JEnv, obj: jobject) -> Result<Self> {
        Ok(Self::from_java(env, obj))
    }
}

impl IntoJava<jobject> for JObject {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jobject {
        self.obj
    }
}

impl<'a> IntoJava<jobject> for &'a JObject {
    #[inline]
    fn into_java(self, _env: &JEnv) -> jobject {
        self.obj
    }
}

impl TryIntoJava<jobject> for JObject {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jobject> {
        Ok(self.obj)
    }
}

impl<'a> TryIntoJava<jobject> for &'a JObject {
    #[inline]
    fn try_into_java(self, _env: &JEnv) -> Result<jobject> {
        Ok(self.obj)
    }
}

impl Into<jobject> for JObject {
    #[inline]
    fn into(self) -> jobject {
        self.obj
    }
}

impl<'a> Into<jobject> for &'a JObject {
    #[inline]
    fn into(self) -> jobject {
        self.obj
    }
}

impl Into<jvalue> for JObject {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.obj }
    }
}

impl<'a> Into<jvalue> for &'a JObject {
    #[inline]
    fn into(self) -> jvalue {
        jvalue { l: self.obj }
    }
}
