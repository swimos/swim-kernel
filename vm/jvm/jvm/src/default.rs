use core::ptr;
use swim_jvm_sys::{jboolean, jbyte, jchar, jshort, jint, jlong, jfloat, jdouble, jobject, JNI_FALSE};

pub trait JDefault: Sized {
    fn default() -> Self;
}

impl JDefault for () {
    #[inline]
    fn default() -> () {
        ()
    }
}

impl JDefault for jboolean {
    #[inline]
    fn default() -> jboolean {
        JNI_FALSE
    }
}

impl JDefault for jbyte {
    #[inline]
    fn default() -> jbyte {
        0
    }
}

impl JDefault for jchar {
    #[inline]
    fn default() -> jchar {
        0
    }
}

impl JDefault for jshort {
    #[inline]
    fn default() -> jshort {
        0
    }
}

impl JDefault for jint {
    #[inline]
    fn default() -> jint {
        0
    }
}

impl JDefault for jlong {
    #[inline]
    fn default() -> jlong {
        0
    }
}

impl JDefault for jfloat {
    #[inline]
    fn default() -> jfloat {
        0.0
    }
}

impl JDefault for jdouble {
    #[inline]
    fn default() -> jdouble {
        0.0
    }
}

impl JDefault for jobject {
    #[inline]
    fn default() -> jobject {
        ptr::null_mut()
    }
}
