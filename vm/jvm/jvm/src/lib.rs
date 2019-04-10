#![no_std]

extern crate swim_codec;
extern crate swim_mem;
#[macro_use]
extern crate swim_c;
extern crate swim_jvm_sys;

pub use error::{Result, JError};
pub use convert::{FromJava, IntoJava, TryFromJava, TryIntoJava};
pub use default::JDefault;
pub use refs::{JGlobalRef, JLocalRef};
pub use types::{JType, JObjectType, JArrayType, JMethodType};
pub use value::JValue;
pub use object::JObject;
pub use array::JArray;
pub use string::JString;
pub use throwable::JThrowable;
pub use class::JClass;
pub use method::JMethod;
pub use field::JField;
pub use env::JEnv;
pub use vm::{JVM, JVMOption, JVMInitArgs, JVMAttachArgs};

pub use swim_jvm_sys::{jboolean, jbyte, jchar, jshort, jint, jlong, jfloat, jdouble, jsize};
pub use swim_jvm_sys::{jobject, jclass, jthrowable, jstring, jarray, jbooleanArray, jbyteArray, jcharArray};
pub use swim_jvm_sys::{jshortArray, jintArray, jlongArray, jfloatArray, jdoubleArray, jobjectArray};
pub use swim_jvm_sys::{jweak, jvalue, jfieldID, jmethodID, jobjectRefType};
pub use swim_jvm_sys::jobjectRefType::*;
pub use swim_jvm_sys::{JNI_OK, JNI_ERR, JNI_EDETACHED, JNI_EVERSION, JNI_ENOMEM, JNI_EEXIST, JNI_EINVAL};
pub use swim_jvm_sys::{JNI_TRUE, JNI_FALSE, JNI_COMMIT, JNI_ABORT};
pub use swim_jvm_sys::{JNINativeMethod, JNIEnv, JavaVM};
pub use swim_jvm_sys::{JNI_VERSION_1_1, JNI_VERSION_1_2, JNI_VERSION_1_4, JNI_VERSION_1_6, JNI_VERSION_1_8, JNI_VERSION_9};

#[macro_use]
mod macros;
mod error;
mod convert;
mod default;
mod refs;
mod types;
mod value;
mod object;
mod array;
mod string;
mod throwable;
mod class;
mod method;
mod field;
mod env;
mod vm;
