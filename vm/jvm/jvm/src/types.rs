use core::borrow::Borrow;
use core::fmt;
use swim_codec::step::In;
use swim_codec::input::{Input, AsInput};
use swim_c::cstr::CStr;
use swim_c::cstring::RawCString;
use crate::error::{Result, JError};

#[derive(PartialEq, Eq, Debug)]
pub enum JType {
    Void,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Object(JObjectType),
    Array(JArrayType),
    Method(JMethodType),
}

#[derive(PartialEq, Eq, Debug)]
pub struct JObjectType {
    sig: RawCString<'static>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JArrayType {
    sig: RawCString<'static>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JMethodType {
    sig: RawCString<'static>,
    ret: usize,
}

impl JType {
    #[doc(hidden)]
    #[inline]
    pub fn object(sig: &str) -> JType {
        JType::Object(JObjectType::new(RawCString::from_copy(sig)))
    }

    #[doc(hidden)]
    #[inline]
    pub fn array(sig: &str) -> JType {
        JType::Array(JArrayType::new(RawCString::from_copy(sig)))
    }

    #[doc(hidden)]
    #[inline]
    pub fn method(sig: &str, ret: usize) -> JType {
        JType::Method(JMethodType::new(RawCString::from_copy(sig), ret))
    }

    pub fn parse<I>(input: I) -> Result<JType> where I: AsInput<Token=char> {
        let mut input = input.as_input();
        match input.head() {
            In('V') => {
                input.step();
                Ok(JType::Void)
            },
            In('Z') => {
                input.step();
                Ok(JType::Boolean)
            },
            In('B') => {
                input.step();
                Ok(JType::Byte)
            },
            In('C') => {
                input.step();
                Ok(JType::Char)
            },
            In('S') => {
                input.step();
                Ok(JType::Short)
            },
            In('I') => {
                input.step();
                Ok(JType::Int)
            },
            In('J') => {
                input.step();
                Ok(JType::Long)
            },
            In('F') => {
                input.step();
                Ok(JType::Float)
            },
            In('D') => {
                input.step();
                Ok(JType::Double)
            },
            In('L') => {
                JObjectType::parse(input).map(|object_type| {
                    JType::Object(object_type)
                })
            },
            In('[') => {
                JArrayType::parse(input).map(|array_type| {
                    JType::Array(array_type)
                })
            },
            In('(') => {
                JMethodType::parse(input).map(|method_type| {
                    JType::Method(method_type)
                })
            },
            _ => Err(JError::from(cstr!("expected type"))),
        }
    }

    pub fn to_class_name(&self) -> RawCString {
        unsafe {
            match *self {
                JType::Void => RawCString::from_copy_unchecked(b"V\0" as &[u8]),
                JType::Boolean => RawCString::from_copy_unchecked(b"Z\0" as &[u8]),
                JType::Byte => RawCString::from_copy_unchecked(b"B\0" as &[u8]),
                JType::Char => RawCString::from_copy_unchecked(b"C\0" as &[u8]),
                JType::Short => RawCString::from_copy_unchecked(b"S\0" as &[u8]),
                JType::Int => RawCString::from_copy_unchecked(b"I\0" as &[u8]),
                JType::Long => RawCString::from_copy_unchecked(b"J\0" as &[u8]),
                JType::Float => RawCString::from_copy_unchecked(b"F\0" as &[u8]),
                JType::Double => RawCString::from_copy_unchecked(b"D\0" as &[u8]),
                JType::Object(ref object_type) => object_type.to_class_name(),
                JType::Array(ref array_type) => array_type.to_class_name(),
                JType::Method(_) => unreachable!(),
            }
        }
    }

    pub fn as_cstr(&self) -> &CStr {
        unsafe {
            match *self {
                JType::Void => CStr::from_bytes_unchecked(b"V\0"),
                JType::Boolean => CStr::from_bytes_unchecked(b"Z\0"),
                JType::Byte => CStr::from_bytes_unchecked(b"B\0"),
                JType::Char => CStr::from_bytes_unchecked(b"C\0"),
                JType::Short => CStr::from_bytes_unchecked(b"S\0"),
                JType::Int => CStr::from_bytes_unchecked(b"I\0"),
                JType::Long => CStr::from_bytes_unchecked(b"J\0"),
                JType::Float => CStr::from_bytes_unchecked(b"F\0"),
                JType::Double => CStr::from_bytes_unchecked(b"D\0"),
                JType::Object(ref object_type) => object_type.as_cstr(),
                JType::Array(ref array_type) => array_type.as_cstr(),
                JType::Method(ref method_type) => method_type.as_cstr(),
            }
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { self.as_cstr().to_str_unchecked() }
    }

    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        unsafe { self.as_cstr().to_bytes() }
    }
}

impl Borrow<CStr> for JType {
    #[inline]
    fn borrow(&self) -> &CStr {
        self.as_cstr()
    }
}

impl fmt::Display for JType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JObjectType {
    #[inline]
    fn new(sig: RawCString<'static>) -> Self {
        Self { sig: sig }
    }

    pub fn parse<I>(input: I) -> Result<JObjectType> where I: AsInput<Token=char> {
        let mut input = input.as_input();
        match input.head() {
            In('L') => input.step(),
            _ => return Err(JError::from(cstr!("expected object type"))),
        }
        let mut sig = RawCString::with_cap(31);
        input.step();
        sig.push('L');
        loop {
            match input.head() {
                In(c) => {
                    input.step();
                    sig.push(c);
                    if c == ';' {
                        return Ok(Self::new(sig));
                    }
                },
                _ => return Err(JError::from(cstr!("invalid object type"))),
            };
        }
    }

    pub fn to_class_name(&self) -> RawCString {
        let len = self.sig.len();
        unsafe { RawCString::from_copy(&self.sig.to_str_unchecked()[1..len.wrapping_sub(1)]) }
    }

    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        self.sig.as_cstr()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { self.sig.to_str_unchecked() }
    }
}

impl Borrow<CStr> for JObjectType {
    #[inline]
    fn borrow(&self) -> &CStr {
        self.as_cstr()
    }
}

impl fmt::Display for JObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JArrayType {
    #[inline]
    fn new(sig: RawCString<'static>) -> Self {
        Self { sig: sig }
    }

    pub fn parse<I>(input: I) -> Result<JArrayType> where I: AsInput<Token=char> {
        let mut input = input.as_input();
        match input.head() {
            In('[') => input.step(),
            _ => return Err(JError::from(cstr!("expected array type"))),
        }
        let mut sig = RawCString::with_cap(31);
        input.step();
        sig.push('[');
        loop {
            if let In(c) = input.head() {
                match c {
                    'V' | 'Z' | 'B' | 'C' | 'S' | 'I' | 'J' | 'F' | 'D' => {
                        input.step();
                        sig.push(c);
                        return Ok(Self::new(sig));
                    },
                    'L' => {
                        input.step();
                        sig.push('L');
                        loop {
                            match input.head() {
                                In(c) => {
                                    input.step();
                                    sig.push(c);
                                    if c == ';' {
                                        return Ok(Self::new(sig));
                                    }
                                },
                                _ => return Err(JError::from(cstr!("invalid array type"))),
                            };
                        }
                    },
                    '[' => {
                        input.step();
                        sig.push('[');
                        continue;
                    },
                    _ => return Err(JError::from(cstr!("invalid array type"))),
                };
            } else {
                return Err(JError::from(cstr!("invalid array type")));
            }
        }
    }

    pub fn to_component_type(&self) -> JType {
        match JType::parse(self.as_str()[1..].as_input()) {
            Ok(component_type) => component_type,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn to_class_name(&self) -> RawCString<'static> {
        unsafe { RawCString::from_copy_unchecked(self.sig.as_cstr()) }
    }

    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        self.sig.as_cstr()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { self.sig.to_str_unchecked() }
    }
}

impl Borrow<CStr> for JArrayType {
    #[inline]
    fn borrow(&self) -> &CStr {
        self.as_cstr()
    }
}

impl fmt::Display for JArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JMethodType {
    #[inline]
    fn new(sig: RawCString<'static>, ret: usize) -> Self {
        Self {
            sig: sig,
            ret: ret,
        }
    }

    pub fn parse<I>(input: I) -> Result<JMethodType> where I: AsInput<Token=char> {
        let mut input = input.as_input();
        match input.head() {
            In('(') => input.step(),
            _ => return Err(JError::from(cstr!("expected method type"))),
        }
        let mut sig = RawCString::with_cap(31);
        input.step();
        sig.push('(');
        loop {
            if let In(c) = input.head() {
                match c {
                    'V' | 'Z' | 'B' | 'C' | 'S' | 'I' | 'J' | 'F' | 'D' => {
                        input.step();
                        sig.push(c);
                        continue;
                    },
                    'L' => {
                        input.step();
                        sig.push('L');
                        loop {
                            match input.head() {
                                In(c) => {
                                    input.step();
                                    sig.push(c);
                                    if c == ';' {
                                        break;
                                    }
                                },
                                _ => return Err(JError::from(cstr!("invalid method type"))),
                            };
                        }
                        continue;
                    },
                    '[' => {
                        input.step();
                        sig.push('[');
                        continue;
                    },
                    ')' => {
                        input.step();
                        sig.push(')');
                        break;
                    },
                    _ => return Err(JError::from(cstr!("invalid method type"))),
                };
            } else {
                return Err(JError::from(cstr!("invalid method type")));
            }
        }
        let ret = sig.len();
        loop {
            if let In(c) = input.head() {
                match c {
                    'V' | 'Z' | 'B' | 'C' | 'S' | 'I' | 'J' | 'F' | 'D' => {
                        input.step();
                        sig.push(c);
                        return Ok(Self::new(sig, ret));
                    },
                    'L' => {
                        input.step();
                        sig.push('L');
                        loop {
                            match input.head() {
                                In(c) => {
                                    input.step();
                                    sig.push(c);
                                    if c == ';' {
                                        return Ok(Self::new(sig, ret));
                                    }
                                },
                                _ => return Err(JError::from(cstr!("invalid method type"))),
                            };
                        }
                    },
                    '[' => {
                        input.step();
                        sig.push('[');
                        continue;
                    },
                    _ => return Err(JError::from(cstr!("invalid method type"))),
                };
            } else {
                return Err(JError::from(cstr!("invalid method type")));
            }
        }
    }

    pub fn to_return_type(&self) -> JType {
        match JType::parse(self.as_str()[self.ret..].as_input()) {
            Ok(return_type) => return_type,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        self.sig.as_cstr()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { self.sig.to_str_unchecked() }
    }
}

impl Borrow<CStr> for JMethodType {
    #[inline]
    fn borrow(&self) -> &CStr {
        self.as_cstr()
    }
}

impl fmt::Display for JMethodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
