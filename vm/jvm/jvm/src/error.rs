use core::result;
use swim_c::cstr::CStr;
use swim_jvm_sys::{JNIEnv, jint};
use crate::throwable::JThrowable;
use crate::env::JEnv;
use crate::convert::FromJava;

pub type Result<T> = result::Result<T, JError>;

#[derive(Clone, Copy, Debug)]
pub enum JError {
    Internal(&'static CStr),
    Exception(JThrowable),
    Code(jint),
}

impl JError {
    pub fn throw(self: JError, env: &JEnv) {
        match self {
            JError::Internal(message) =>
                if let Ok(cls) = env.find_class(cstr!("java/lang/InternalError")) {
                    env.throw_new(cls, message);
                },
            JError::Exception(throwable) =>
                if env.exception_occurred().is_none() {
                    env.throw(throwable);
                },
            JError::Code(_) => panic!(),
        }
    }

    pub fn panic<E>(env: E) where E: Into<*mut JNIEnv> {
        let env = env.into();
        jni!(env, ExceptionDescribe);
        panic!();
    }
}

impl<'a> From<&'static CStr> for JError {
    #[inline]
    fn from(message: &'static CStr) -> JError {
        JError::Internal(message)
    }
}

impl<'a> From<*mut JNIEnv> for JError {
    #[inline]
    fn from(env: *mut JNIEnv) -> JError {
        let exception = JThrowable::from_java(&JEnv::new(env), jni!(env, ExceptionOccurred));
        JError::Exception(exception)
    }
}

impl From<jint> for JError {
    #[inline]
    fn from(code: jint) -> JError {
        JError::Code(code)
    }
}
