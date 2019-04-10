use swim_jvm_sys::jfieldID;
use crate::env::JEnv;
use crate::convert::FromJava;

#[derive(Clone, Copy)]
pub struct JField {
    env: JEnv,
    fid: jfieldID,
}

impl JField {
    #[inline]
    pub fn get_env(&self) -> &JEnv {
        &self.env
    }
}

impl FromJava<jfieldID> for JField {
    #[inline]
    fn from_java(env: &JEnv, fid: jfieldID) -> Self {
        Self {
            env: env.clone(),
            fid: fid,
        }
    }
}

impl Into<jfieldID> for JField {
    #[inline]
    fn into(self) -> jfieldID {
        self.fid
    }
}

impl<'a> Into<jfieldID> for &'a JField {
    #[inline]
    fn into(self) -> jfieldID {
        self.fid
    }
}
