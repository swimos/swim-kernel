use swim_jvm_sys::{jboolean, jbyte, jchar, jshort, jint, jlong, jfloat, jdouble, jvalue};
use crate::object::JObject;
use crate::array::JArray;
use crate::string::JString;
use crate::throwable::JThrowable;
use crate::class::JClass;

#[derive(Clone, Copy, Debug)]
pub enum JValue {
    Void,
    Boolean(jboolean),
    Byte(jbyte),
    Char(jchar),
    Short(jshort),
    Int(jint),
    Long(jlong),
    Float(jfloat),
    Double(jdouble),
    Object(JObject),
}

impl From<()> for JValue {
    #[inline]
    fn from(_: ()) -> JValue {
        JValue::Void
    }
}

impl From<bool> for JValue {
    #[inline]
    fn from(z: bool) -> JValue {
        JValue::Boolean(z as jboolean)
    }
}

impl From<jbyte> for JValue {
    #[inline]
    fn from(b: jbyte) -> JValue {
        JValue::Byte(b)
    }
}

impl From<jchar> for JValue {
    #[inline]
    fn from(c: jchar) -> JValue {
        JValue::Char(c)
    }
}

impl From<jshort> for JValue {
    #[inline]
    fn from(s: jshort) -> JValue {
        JValue::Short(s)
    }
}

impl From<jint> for JValue {
    #[inline]
    fn from(i: jint) -> JValue {
        JValue::Int(i)
    }
}

impl From<jlong> for JValue {
    #[inline]
    fn from(j: jlong) -> JValue {
        JValue::Long(j)
    }
}

impl From<jfloat> for JValue {
    #[inline]
    fn from(f: jfloat) -> JValue {
        JValue::Float(f)
    }
}

impl From<jdouble> for JValue {
    #[inline]
    fn from(d: jdouble) -> JValue {
        JValue::Double(d)
    }
}

impl From<JObject> for JValue {
    #[inline]
    fn from(l: JObject) -> JValue {
        JValue::Object(l)
    }
}

impl From<JArray> for JValue {
    #[inline]
    fn from(a: JArray) -> JValue {
        JValue::Object(a.into())
    }
}

impl From<JString> for JValue {
    #[inline]
    fn from(s: JString) -> JValue {
        JValue::Object(s.into())
    }
}

impl From<JThrowable> for JValue {
    #[inline]
    fn from(t: JThrowable) -> JValue {
        JValue::Object(t.into())
    }
}

impl From<JClass> for JValue {
    #[inline]
    fn from(k: JClass) -> JValue {
        JValue::Object(k.into())
    }
}

impl Into<jvalue> for JValue {
    fn into(self) -> jvalue {
        match self {
            JValue::Void => jvalue { z: 0 },
            JValue::Boolean(z) => jvalue { z: z },
            JValue::Byte(b) => jvalue { b: b },
            JValue::Char(c) => jvalue { c: c },
            JValue::Short(s) => jvalue { s: s },
            JValue::Int(i) => jvalue { i: i },
            JValue::Long(j) => jvalue { j: j },
            JValue::Float(f) => jvalue { f: f },
            JValue::Double(d) => jvalue { d: d },
            JValue::Object(l) => jvalue { l: l.into() },
        }
    }
}
