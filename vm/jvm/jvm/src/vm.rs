use core::ptr;
use swim_mem::alloc::{HoldError, TryClone};
use swim_mem::lease::RawBuf;
use swim_c::void;
use swim_c::cstr::CStr;
use swim_c::cstring::RawCString;
use swim_jvm_sys::{JavaVM, JNIEnv, JNI_TRUE, JNI_VERSION_1_8};
use swim_jvm_sys::{JavaVMOption, JavaVMInitArgs, JavaVMAttachArgs};
use swim_jvm_sys::{jint, jobject};
use crate::error::{Result, JError};
use crate::env::JEnv;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct JVM {
    vm: *mut JavaVM,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JVMOption {
    pub option_string: RawCString<'static>,
    pub extra_info: *const void,
}

impl TryClone for JVMOption {
    #[inline]
    fn try_clone(&self) -> core::result::Result<JVMOption, HoldError> {
        Ok(self.clone())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JVMInitArgs {
    pub version: jint,
    pub options: RawBuf<'static, JVMOption>,
    pub ignore_unrecognized: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JVMAttachArgs {
    pub version: jint,
    pub name: RawCString<'static>,
    pub group: jobject,
}

impl JVM {
    #[inline]
    pub fn new(vm: *mut JavaVM) -> Self {
        Self { vm: vm }
    }

    #[cfg(feature = "jvm")]
    pub fn get_default_java_vm_init_args(version: jint) -> JVMInitArgs {
        let mut args = JavaVMInitArgs {
            version: version,
            nOptions: 0,
            options: ptr::null(),
            ignoreUnrecognized: JNI_TRUE,
        };
        let res = unsafe { swim_jvm_sys::JNI_GetDefaultJavaVMInitArgs(&mut args) };
        if res < 0 {
            panic!();
        }
        JVMInitArgs::from(args)
    }

    #[cfg(feature = "jvm")]
    pub fn create_java_vm(init_args: Option<JVMInitArgs>) -> (JVM, JEnv) {
        let mut pvm: *mut JavaVM = ptr::null_mut();
        let mut penv: *mut JNIEnv = ptr::null_mut();
        let mut args = JavaVMInitArgs {
            version: JNI_VERSION_1_8,
            nOptions: 0,
            options: ptr::null(),
            ignoreUnrecognized: JNI_TRUE,
        };
        if let Some(init_args) = init_args {
            // options must live as long as args
            args.version = init_args.version;
            let options = init_args.as_java_vm_options();
            args.nOptions = options.len() as jint;
            args.options = options.as_ptr();
            args.ignoreUnrecognized = init_args.ignore_unrecognized as crate::swim_jvm_sys::jboolean;
            let res = unsafe { swim_jvm_sys::JNI_CreateJavaVM(&mut pvm, &mut penv, &mut args) };
            if res < 0 {
                panic!();
            }
            (JVM::new(pvm), JEnv::new(penv))
        } else {
            let res = unsafe { swim_jvm_sys::JNI_GetDefaultJavaVMInitArgs(&mut args) };
            if res < 0 {
                panic!();
            }
            let res = unsafe { swim_jvm_sys::JNI_CreateJavaVM(&mut pvm, &mut penv, &mut args) };
            if res < 0 {
                panic!();
            }
            (JVM::new(pvm), JEnv::new(penv))
        }
    }

    pub fn attach_current_thread(&self, attach_args: Option<JVMAttachArgs>) -> Result<JEnv> {
        let mut penv: *mut JNIEnv = ptr::null_mut();
        let attach_args = attach_args.unwrap_or_else(|| JVMAttachArgs::default());
        let args = (&attach_args).into();
        let res = unsafe { ((**self.vm).AttachCurrentThread)(self.vm, &mut penv, &args) };
        if res == 0 {
            Ok(JEnv::new(penv))
        } else {
            Err(JError::from(res))
        }
    }

    pub fn attach_current_thread_as_daemon(&self, attach_args: Option<JVMAttachArgs>) -> Result<JEnv> {
        let mut penv: *mut JNIEnv = ptr::null_mut();
        let attach_args = attach_args.unwrap_or_else(|| JVMAttachArgs::default());
        let args = (&attach_args).into();
        let res = unsafe { ((**self.vm).AttachCurrentThreadAsDaemon)(self.vm, &mut penv, &args) };
        if res == 0 {
            Ok(JEnv::new(penv))
        } else {
            Err(JError::from(res))
        }
    }

    pub fn detach_current_thread(&self) {
        let res = unsafe { ((**self.vm).DetachCurrentThread)(self.vm) };
        if res < 0 {
            panic!()
        }
    }

    pub fn get_env(&self, version: jint) -> Result<JEnv> {
        let mut penv: *mut JNIEnv = ptr::null_mut();
        let res = unsafe { ((**self.vm).GetEnv)(self.vm, &mut penv, version) };
        if res == 0 {
            Ok(JEnv::new(penv))
        } else {
            Err(JError::from(res))
        }
    }

    pub fn destroy_java_vm(self) {
        let res = unsafe { ((**self.vm).DestroyJavaVM)(self.vm) };
        if res < 0 {
            panic!()
        }
    }
}

impl JVMOption {
    pub fn new<S>(option_string: S) -> Self where S: Into<RawCString<'static>> {
        let option_string = option_string.into();
        Self {
            option_string: option_string,
            extra_info: ptr::null(),
        }
    }

    pub fn with_extra_info<S>(option_string: S, extra_info: *mut void) -> Self
            where S: Into<RawCString<'static>> {
        let option_string = option_string.into();
        Self {
            option_string: option_string,
            extra_info: extra_info,
        }
    }

    fn as_java_vm_option(&self) -> JavaVMOption {
        JavaVMOption {
            optionString: self.option_string.as_cptr(),
            extraInfo: self.extra_info,
        }
    }
}

impl From<JavaVMOption> for JVMOption {
    fn from(option: JavaVMOption) -> Self {
        Self {
            option_string: unsafe { RawCString::from_copy_unchecked(CStr::from_cptr(option.optionString)) },
            extra_info: option.extraInfo,
        }
    }
}

impl JVMInitArgs {
    pub fn new(version: jint, options: RawBuf<'static, JVMOption>, ignore_unrecognized: bool) -> Self {
        Self {
            version: version,
            options: options,
            ignore_unrecognized: ignore_unrecognized,
        }
    }

    pub fn as_java_vm_options(&self) -> RawBuf<'static, JavaVMOption> {
        let mut options = RawBuf::with_cap(self.options.len());
        for option in self.options.iter() {
            options.push(option.as_java_vm_option());
        }
        options
    }
}

impl From<JavaVMInitArgs> for JVMInitArgs {
    fn from(args: JavaVMInitArgs) -> Self {
        let n = args.nOptions;
        let mut options = RawBuf::with_cap(n as usize);
        for i in 0..n {
            let option = unsafe { *args.options.offset(i as isize) };
            options.push(JVMOption::from(option));
        }
        Self {
            version: args.version,
            options: options,
            ignore_unrecognized: args.ignoreUnrecognized == JNI_TRUE,
        }
    }
}

impl JVMAttachArgs {
    pub fn new<S, O>(version: jint, name: S, group: O) -> Self
            where S: Into<RawCString<'static>>, O: Into<jobject> {
        let name = name.into();
        let group = group.into();
        Self {
            version: version,
            name: name,
            group: group,
        }
    }
}

impl<'a> Into<JavaVMAttachArgs> for &'a JVMAttachArgs {
    fn into(self) -> JavaVMAttachArgs {
        JavaVMAttachArgs {
            version: self.version,
            name: self.name.as_cptr(),
            group: self.group,
        }
    }
}

impl Default for JVMAttachArgs {
    fn default() -> JVMAttachArgs {
        let name = RawCString::from_copy("swim");
        JVMAttachArgs::new(JNI_VERSION_1_8, name, ptr::null_mut())
    }
}
