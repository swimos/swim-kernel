use swim_jvm_sys::jobject;
use crate::object::JObject;
use crate::env::JEnv;
use crate::convert::IntoJava;

pub struct JGlobalRef {
    env: JEnv,
    gref: jobject,
}

unsafe impl Send for JGlobalRef {}
unsafe impl Sync for JGlobalRef {}

impl JGlobalRef {
    #[inline]
    pub unsafe fn new(env: &JEnv, gref: jobject) -> Self {
        Self {
            env: env.clone(),
            gref: gref,
        }
    }
}

impl Drop for JGlobalRef {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.env.delete_global_ref(self.gref) };
    }
}

impl Into<JObject> for JGlobalRef {
    #[inline]
    fn into(self) -> JObject {
        self.gref.into_java(&self.env)
    }
}

impl Into<jobject> for JGlobalRef {
    #[inline]
    fn into(self) -> jobject {
        self.gref
    }
}

pub struct JLocalRef {
    env: JEnv,
    lref: jobject,
}

impl JLocalRef {
    #[inline]
    pub unsafe fn new(env: &JEnv, lref: jobject) -> Self {
        Self {
            env: env.clone(),
            lref: lref,
        }
    }
}

impl Drop for JLocalRef {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.env.delete_local_ref(self.lref) };
    }
}

impl Into<JObject> for JLocalRef {
    #[inline]
    fn into(self) -> JObject {
        self.lref.into_java(&self.env)
    }
}

impl Into<jobject> for JLocalRef {
    #[inline]
    fn into(self) -> jobject {
        self.lref
    }
}
