use swim_jvm_sys::jmethodID;
use crate::env::JEnv;
use crate::convert::FromJava;

#[derive(Clone, Copy)]
pub struct JMethod {
    env: JEnv,
    mid: jmethodID,
}

impl JMethod {
    #[inline]
    pub fn get_env(&self) -> &JEnv {
        &self.env
    }
}

impl FromJava<jmethodID> for JMethod {
    #[inline]
    fn from_java(env: &JEnv, mid: jmethodID) -> Self {
        Self {
            env: env.clone(),
            mid: mid,
        }
    }
}

impl Into<jmethodID> for JMethod {
    #[inline]
    fn into(self) -> jmethodID {
        self.mid
    }
}

impl<'a> Into<jmethodID> for &'a JMethod {
    #[inline]
    fn into(self) -> jmethodID {
        self.mid
    }
}
