use core::cmp;
use core::fmt;
use core::hash;
use core::i8;
use core::i16;
use core::i32;
use core::i64;
use core::marker::PhantomData;
use core::mem;
use core::num::NonZeroU64;
use core::ptr;
use core::u8;
use core::u16;
use core::u32;
use core::u64;
use swim_core::f16;
use swim_mem::alloc::{Hold, HoldError, Stow, TryClone, CloneIntoHold};
use crate::item::{Item, Value};

/// `Value` variant representing a number.
#[derive(Eq)]
#[repr(C)]
pub struct Num<'a> {
    /// Discriminant with a type between `NUMBER_TYPE_MIN` and `NUMBER_TYPE_MAX`
    /// at the lowest byte address.
    ///
    /// ```text
    /// 0        1        2        3        4        5        6        7        8
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// |  type  |      0 |      0 |      0 |      0 |      0 |      0 |      0 |
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// ```
    _0: NonZeroU64,
    _1: u64,
    /// Variant over allocation lifetime.
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Num<'a> {
    /// Constructs a new `Num` from a `u8` value.
    pub const fn from_u8(value: u8) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::U8_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from an `i8` value.
    pub const fn from_i8(value: i8) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::I8_TYPE)) },
            _1: value as i64 as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from a `u16` value.
    pub const fn from_u16(value: u16) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::U16_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from an `i16` value.
    pub const fn from_i16(value: i16) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::I16_TYPE)) },
            _1: value as i64 as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from a `u32` value.
    pub const fn from_u32(value: u32) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::U32_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from an `i32` value.
    pub const fn from_i32(value: i32) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::I32_TYPE)) },
            _1: value as i64 as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from a `u64` value.
    pub const fn from_u64(value: u64) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::U64_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from an `i64` value.
    pub const fn from_i64(value: i64) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::I64_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from an `f16` value.
    pub fn from_f16(value: f16) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::F16_TYPE)) },
            _1: unsafe { mem::transmute::<f16, u16>(value) as u64 },
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from an `f32` value.
    pub fn from_f32(value: f32) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::F32_TYPE)) },
            _1: unsafe { mem::transmute::<f32, u32>(value) as u64 },
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from an `f64` value.
    pub fn from_f64(value: f64) -> Num<'a> {
        Num {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::F64_TYPE)) },
            _1: unsafe { mem::transmute::<f64, u64>(value) },
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Num` from a `usize` value.
    #[cfg(target_pointer_width = "32")]
    pub const fn from_usize(value: usize) -> Num<'a> {
        Num::from_u32(value as u32)
    }
    /// Constructs a new `Num` from a `usize` value.
    #[cfg(target_pointer_width = "64")]
    pub const fn from_usize(value: usize) -> Num<'a> {
        Num::from_u64(value as u64)
    }

    /// Constructs a new `Num` from an `isize` value.
    #[cfg(target_pointer_width = "32")]
    pub const fn from_isize(value: isize) -> Num<'a> {
        Num::from_i32(value as i32)
    }
    /// Constructs a new `Num` from an `isize` value.
    #[cfg(target_pointer_width = "64")]
    pub const fn from_isize(value: isize) -> Num<'a> {
        Num::from_i64(value as i64)
    }

    /// Returns a pointer to the tag in the first byte of this `Num`.
    #[inline(always)]
    pub(crate) unsafe fn tag_ptr(&self) -> *mut u8 {
        mem::transmute::<&Num<'a>, *mut u8>(self)
    }

    /// Returns the tag from the first byte of this `Num`.
    #[inline(always)]
    pub(crate) fn tag(&self) -> u8 {
        unsafe { *self.tag_ptr() }
    }

    /// Returns the type tag from the low 7 bits of the first byte of this `Num`.
    #[inline(always)]
    pub(crate) fn type_tag(&self) -> u8 {
        self.tag() & Value::TYPE_MASK
    }

    /// Upcasts this `Num` reference to a `Value` reference.
    #[inline]
    pub fn as_value(&self) -> &Value<'a> {
        unsafe { mem::transmute::<&Num<'a>, &Value<'a>>(self) }
    }

    /// Upcasts this `Num` reference to a mutable `Value` reference.
    #[inline]
    pub fn as_mut_value(&mut self) -> &mut Value<'a> {
        unsafe { mem::transmute::<&mut Num<'a>, &mut Value<'a>>(self) }
    }

    /// Upcasts this `Num` to a `Value`.
    #[inline]
    pub fn into_value(self) -> Value<'a> {
        unsafe { mem::transmute::<Num<'a>, Value<'a>>(self) }
    }

    /// Upcasts this `Num` to an `Item`.
    #[inline]
    pub fn into_item(self) -> Item<'a> {
        Item::from_value(self.into_value())
    }

    /// Returns `true` if this `Num` was created from a `u8` value.
    pub fn is_u8(&self) -> bool {
        self.type_tag() == Value::U8_TYPE
    }

    /// Returns `true` if this `Num` was created from an `i8` value.
    pub fn is_i8(&self) -> bool {
        self.type_tag() == Value::I8_TYPE
    }

    /// Returns `true` if this `Num` was created from a `u16` value.
    pub fn is_u16(&self) -> bool {
        self.type_tag() == Value::U16_TYPE
    }

    /// Returns `true` if this `Num` was created from an `i16` value.
    pub fn is_i16(&self) -> bool {
        self.type_tag() == Value::I16_TYPE
    }

    /// Returns `true` if this `Num` was created from a `u32` value.
    pub fn is_u32(&self) -> bool {
        self.type_tag() == Value::U32_TYPE
    }

    /// Returns `true` if this `Num` was created from an `i32` value.
    pub fn is_i32(&self) -> bool {
        self.type_tag() == Value::I32_TYPE
    }

    /// Returns `true` if this `Num` was created from a `u64` value.
    pub fn is_u64(&self) -> bool {
        self.type_tag() == Value::U64_TYPE
    }

    /// Returns `true` if this `Num` was created from an `i64` value.
    pub fn is_i64(&self) -> bool {
        self.type_tag() == Value::I64_TYPE
    }

    /// Returns `true` if this `Num` was created from an `f16` value.
    pub fn is_f16(&self) -> bool {
        self.type_tag() == Value::F16_TYPE
    }

    /// Returns `true` if this `Num` was created from an `f32` value.
    pub fn is_f32(&self) -> bool {
        self.type_tag() == Value::F32_TYPE
    }

    /// Returns `true` if this `Num` was created from an `f64` value.
    pub fn is_f64(&self) -> bool {
        self.type_tag() == Value::F64_TYPE
    }

    /// Returns `true` if this `Num` was created from a big integer.
    pub fn is_big_int(&self) -> bool {
        self.type_tag() == Value::BIG_INT_TYPE
    }

    /// Returns `true` if this `Num` was created from a big decimal.
    pub fn is_big_dec(&self) -> bool {
        self.type_tag() == Value::BIG_DEC_TYPE
    }

    /// Returns `true` if this `Num` was created from a `usize` value.
    #[inline]
    pub fn is_usize(&self) -> bool {
        #[cfg(target_pointer_width = "32")]
        let is = self.is_u32();
        #[cfg(target_pointer_width = "64")]
        let is = self.is_u64();
        is
    }

    /// Returns `true` if this `Num` was created from an `isize` value.
    #[inline]
    pub fn is_isize(&self) -> bool {
        #[cfg(target_pointer_width = "32")]
        let is = self.is_i32();
        #[cfg(target_pointer_width = "64")]
        let is = self.is_i64();
        is
    }

    /// Returns `true` if this `Num` can losslessly convert to a `u8` value.
    pub fn is_valid_u8(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE => true,
            Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                self._1 <= u8::MAX as u64
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                u8::MIN as i64 <= self._1 as i64 && self._1 as i64 <= u8::MAX as i64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value == f16::from(f32::from(value) as u8 as f32)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == value as u8 as f32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as u8 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to an `i8` value.
    pub fn is_valid_i8(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                self._1 <= i8::MAX as u64
            },
            Value::I8_TYPE => true,
            Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                i8::MIN as i64 <= self._1 as i64 && self._1 as i64 <= i8::MAX as i64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value == f16::from(f32::from(value) as i8 as f32)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == value as i8 as f32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as i8 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to a `u16` value.
    pub fn is_valid_u16(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE => true,
            Value::U32_TYPE | Value::U64_TYPE => {
                self._1 <= u16::MAX as u64
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                u16::MIN as i64 <= self._1 as i64 && self._1 as i64 <= u16::MAX as i64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value == f16::from(f32::from(value) as u16 as f32)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == value as u16 as f32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as u16 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to an `i16` value.
    pub fn is_valid_i16(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE => true,
            Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                self._1 <= i16::MAX as u64
            },
            Value::I8_TYPE | Value::I16_TYPE => true,
            Value::I32_TYPE | Value::I64_TYPE => {
                i16::MIN as i64 <= self._1 as i64 && self._1 as i64 <= i16::MAX as i64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value == f16::from(f32::from(value) as i16 as f32)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == value as i16 as f32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as i16 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to a `u32` value.
    pub fn is_valid_u32(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE => true,
            Value::U64_TYPE => {
                self._1 <= u32::MAX as u64
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                u32::MIN as i64 <= self._1 as i64 && self._1 as i64 <= u32::MAX as i64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value == f16::from(f32::from(value) as u32 as f32)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == value as u32 as f32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as u32 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to an `i32` value.
    pub fn is_valid_i32(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE => true,
            Value::U32_TYPE | Value::U64_TYPE => {
                self._1 <= i32::MAX as u64
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE => true,
            Value::I64_TYPE => {
                i32::MIN as i64 <= self._1 as i64 && self._1 as i64 <= i32::MAX as i64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value == f16::from(f32::from(value) as i32 as f32)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == value as i32 as f32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as i32 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to a `u64` value.
    pub fn is_valid_u64(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => true,
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                u64::MIN as i64 <= self._1 as i64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value == f16::from(f32::from(value) as u64 as f32)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == value as u64 as f32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as u64 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to an `i64` value.
    pub fn is_valid_i64(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE => true,
            Value::U64_TYPE => {
                self._1 <= i64::MAX as u64
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => true,
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value == f16::from(f32::from(value) as i64 as f32)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == value as i64 as f32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as i64 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to an `f16` value.
    pub fn is_valid_f16(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE => true,
            Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                self._1 == f32::from(f16::from(self._1 as f32)) as u64
            },
            Value::I8_TYPE => true,
            Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as i64 == f32::from(f16::from(self._1 as i64 as f32)) as i64
            },
            Value::F16_TYPE => true,
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value == f32::from(f16::from(value))
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == f64::from(f16::from(value))
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to an `f32` value.
    pub fn is_valid_f32(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE => true,
            Value::U32_TYPE | Value::U64_TYPE => {
                self._1 == self._1 as f32 as u64
            },
            Value::I8_TYPE | Value::I16_TYPE => true,
            Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as i64 == self._1 as i64 as f32 as i64
            },
            Value::F16_TYPE | Value::F32_TYPE => true,
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value == value as f32 as f64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to an `f64` value.
    pub fn is_valid_f64(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE => true,
            Value::U64_TYPE => {
                self._1 == self._1 as f64 as u64
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE => true,
            Value::I64_TYPE => {
                self._1 as i64 == self._1 as i64 as f64 as i64
            },
            Value::F16_TYPE | Value::F32_TYPE | Value::F64_TYPE => true,
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to a big integer.
    pub fn is_valid_big_int(&self) -> bool {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => true,
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => true,
            Value::F16_TYPE | Value::F32_TYPE | Value::F64_TYPE => unimplemented!(),
            Value::BIG_INT_TYPE => true,
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Returns `true` if this `Num` can losslessly convert to a big decimal.
    pub fn is_valid_big_dec(&self) -> bool {
        true
    }

    /// Returns `true` if this `Num` can losslessly convert to a `usize` value.
    #[inline]
    pub fn is_valid_usize(&self) -> bool {
        #[cfg(target_pointer_width = "32")]
        let is = self.is_valid_u32();
        #[cfg(target_pointer_width = "64")]
        let is = self.is_valid_u64();
        is
    }

    /// Returns `true` if this `Num` can losslessly convert to an `isize` value.
    #[inline]
    pub fn is_valid_isize(&self) -> bool {
        #[cfg(target_pointer_width = "32")]
        let is = self.is_valid_i32();
        #[cfg(target_pointer_width = "64")]
        let is = self.is_valid_i64();
        is
    }

    /// Coerces this `Num` to a `u8` value.
    pub fn as_u8(&self) -> u8 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as u8
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value) as u8
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as u8
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as u8
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to an `i8` value.
    pub fn as_i8(&self) -> i8 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as i8
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value) as i8
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as i8
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as i8
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to a `u16` value.
    pub fn as_u16(&self) -> u16 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as u16
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value) as u16
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as u16
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as u16
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to an `i16` value.
    pub fn as_i16(&self) -> i16 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as i16
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value) as i16
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as i16
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as i16
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to a `u32` value.
    pub fn as_u32(&self) -> u32 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as u32
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value) as u32
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as u32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as u32
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to an `i32` value.
    pub fn as_i32(&self) -> i32 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as i32
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value) as i32
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as i32
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as i32
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to a `u64` value.
    pub fn as_u64(&self) -> u64 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value) as u64
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as u64
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as u64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to an `i64` value.
    pub fn as_i64(&self) -> i64 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as i64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value) as i64
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as i64
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as i64
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to an `f16` value.
    pub fn as_f16(&self) -> f16 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                f16::from(self._1 as f32)
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                f16::from(self._1 as i64 as f32)
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                f16::from(value)
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                f16::from(value)
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to an `f32` value.
    pub fn as_f32(&self) -> f32 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                self._1 as f32
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as i64 as f32
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f32::from(value)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value as f32
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to an `f64` value.
    pub fn as_f64(&self) -> f64 {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                self._1 as f64
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                self._1 as i64 as f64
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                f64::from(value)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value as f64
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Coerces this `Num` to a `usize` value.
    #[inline]
    pub fn as_usize(&self) -> usize {
        #[cfg(target_pointer_width = "32")]
        let value = self.as_u32();
        #[cfg(target_pointer_width = "64")]
        let value = self.as_u64();
        value as usize
    }

    /// Coerces this `Num` to an `isize` value.
    #[inline]
    pub fn as_isize(&self) -> isize {
        #[cfg(target_pointer_width = "32")]
        let value = self.as_i32();
        #[cfg(target_pointer_width = "64")]
        let value = self.as_i64();
        value as isize
    }

    /// Losslessly converts this `Num` to a `u8` value, if possible.
    pub fn to_u8(&self) -> Option<u8> {
        match self.type_tag() {
            Value::U8_TYPE => {
                Some(self._1 as u8)
            },
            Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                if self._1 <= u8::MAX as u64 {
                    Some(self._1 as u8)
                } else {
                    None
                }
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                if u8::MIN as i64 <= self._1 as i64 && self._1 as i64 <= u8::MAX as i64 {
                    Some(self._1 as u8)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                if value == f16::from(f32::from(value) as u8 as f32) {
                    Some(f32::from(value) as u8)
                } else {
                    None
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == value as u8 as f32 {
                    Some(value as u8)
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as u8 as f64 {
                    Some(value as u8)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to an `i8` value, if possible.
    pub fn to_i8(&self) -> Option<i8> {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                if self._1 <= i8::MAX as u64 {
                    Some(self._1 as i8)
                } else {
                    None
                }
            },
            Value::I8_TYPE => {
                Some(self._1 as i8)
            },
            Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                if i8::MIN as i64 <= self._1 as i64 && self._1 as i64 <= i8::MAX as i64 {
                    Some(self._1 as i8)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                if value == f16::from(f32::from(value) as i8 as f32) {
                    Some(f32::from(value) as i8)
                } else {
                    None
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == value as i8 as f32 {
                    Some(value as i8)
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as i8 as f64 {
                    Some(value as i8)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to a `u16` value, if possible.
    pub fn to_u16(&self) -> Option<u16> {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE => {
                Some(self._1 as u16)
            },
            Value::U32_TYPE | Value::U64_TYPE => {
                if self._1 <= u16::MAX as u64 {
                    Some(self._1 as u16)
                } else {
                    None
                }
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                if u16::MIN as i64 <= self._1 as i64 && self._1 as i64 <= u16::MAX as i64 {
                    Some(self._1 as u16)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                if value == f16::from(f32::from(value) as u16 as f32) {
                    Some(f32::from(value) as u16)
                } else {
                    None
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == value as u16 as f32 {
                    Some(value as u16)
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as u16 as f64 {
                    Some(value as u16)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to an `i16` value, if possible.
    pub fn to_i16(&self) -> Option<i16> {
        match self.type_tag() {
            Value::U8_TYPE => {
                Some(self._1 as i16)
            },
            Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                if self._1 <= i16::MAX as u64 {
                    Some(self._1 as i16)
                } else {
                    None
                }
            },
            Value::I8_TYPE | Value::I16_TYPE => {
                Some(self._1 as i16)
            },
            Value::I32_TYPE | Value::I64_TYPE => {
                if i16::MIN as i64 <= self._1 as i64 && self._1 as i64 <= i16::MAX as i64 {
                    Some(self._1 as i16)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                if value == f16::from(f32::from(value) as i16 as f32) {
                    Some(f32::from(value) as i16)
                } else {
                    None
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == value as i16 as f32 {
                    Some(value as i16)
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as i16 as f64 {
                    Some(value as i16)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to a `u32` value, if possible.
    pub fn to_u32(&self) -> Option<u32> {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE => {
                Some(self._1 as u32)
            },
            Value::U64_TYPE => {
                if self._1 <= u32::MAX as u64 {
                    Some(self._1 as u32)
                } else {
                    None
                }
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                if u32::MIN as i64 <= self._1 as i64 && self._1 as i64 <= u32::MAX as i64 {
                    Some(self._1 as u32)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                if value == f16::from(f32::from(value) as u32 as f32) {
                    Some(f32::from(value) as u32)
                } else {
                    None
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == value as u32 as f32 {
                    Some(value as u32)
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as u32 as f64 {
                    Some(value as u32)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to an `i32` value, if possible.
    pub fn to_i32(&self) -> Option<i32> {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE => {
                Some(self._1 as i32)
            },
            Value::U32_TYPE | Value::U64_TYPE => {
                if self._1 <= i32::MAX as u64 {
                    Some(self._1 as i32)
                } else {
                    None
                }
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE => {
                Some(self._1 as i32)
            },
            Value::I64_TYPE => {
                if i32::MIN as i64 <= self._1 as i64 && self._1 as i64 <= i32::MAX as i64 {
                    Some(self._1 as i32)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                if value == f16::from(f32::from(value) as i32 as f32) {
                    Some(f32::from(value) as i32)
                } else {
                    None
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == value as i32 as f32 {
                    Some(value as i32)
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as i32 as f64 {
                    Some(value as i32)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to a `u64` value, if possible.
    pub fn to_u64(&self) -> Option<u64> {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                Some(self._1)
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                if u64::MIN as i64 <= self._1 as i64 {
                    Some(self._1)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                if value == f16::from(f32::from(value) as u64 as f32) {
                    Some(f32::from(value) as u64)
                } else {
                    None
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == value as u64 as f32 {
                    Some(self._1 as u64)
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as u64 as f64 {
                    Some(self._1 as u64)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to an `i64` value, if possible.
    pub fn to_i64(&self) -> Option<i64> {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE => {
                Some(self._1 as i64)
            },
            Value::U64_TYPE => {
                if self._1 <= i64::MAX as u64 {
                    Some(self._1 as i64)
                } else {
                    None
                }
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                Some(self._1 as i64)
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                if value == f16::from(f32::from(value) as i64 as f32) {
                    Some(f32::from(value) as i64)
                } else {
                    None
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == value as i64 as f32 {
                    Some(value as i64)
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as i64 as f64 {
                    Some(value as i64)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to an `f16` value, if possible.
    pub fn to_f16(&self) -> Option<f16> {
        match self.type_tag() {
            Value::U8_TYPE => {
                Some(f16::from(self._1 as u8))
            },
            Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                if self._1 == f32::from(f16::from(self._1 as f32)) as u64 {
                    Some(f16::from(self._1 as f32))
                } else {
                    None
                }
            },
            Value::I8_TYPE => {
                Some(f16::from(self._1 as i8))
            },
            Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                if self._1 as i64 == f32::from(f16::from(self._1 as i64 as f32)) as i64 {
                    Some(f16::from(self._1 as i64 as f32))
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                Some(value)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                if value == f32::from(f16::from(value)) {
                    Some(f16::from(value))
                } else {
                    None
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == f64::from(f16::from(value)) {
                    Some(f16::from(value))
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to an `f32` value, if possible.
    pub fn to_f32(&self) -> Option<f32> {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE => {
                Some(self._1 as f32)
            },
            Value::U32_TYPE | Value::U64_TYPE => {
                if self._1 == self._1 as f32 as u64 {
                    Some(self._1 as f32)
                } else {
                    None
                }
            },
            Value::I8_TYPE | Value::I16_TYPE => {
                Some(self._1 as i64 as f32)
            },
            Value::I32_TYPE | Value::I64_TYPE => {
                if self._1 as i64 == self._1 as i64 as f32 as i64 {
                    Some(self._1 as i64 as f32)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                Some(f32::from(value))
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                Some(value)
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as f32 as f64 {
                    Some(value as f32)
                } else {
                    None
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to an `f64` value, if possible.
    pub fn to_f64(&self) -> Option<f64> {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE => {
                Some(self._1 as f64)
            },
            Value::U64_TYPE => {
                if self._1 == self._1 as f64 as u64 {
                    Some(self._1 as f64)
                } else {
                    None
                }
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE => {
                Some(self._1 as i64 as f64)
            },
            Value::I64_TYPE => {
                if self._1 as i64 == self._1 as i64 as f64 as i64 {
                    Some(self._1 as i64 as f64)
                } else {
                    None
                }
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                Some(f64::from(value))
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                Some(value as f64)
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                Some(value)
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }

    /// Losslessly converts this `Num` to a `usize` value, if possible.
    #[inline]
    pub fn to_usize(&self) -> Option<usize> {
        #[cfg(target_pointer_width = "32")]
        let value = self.to_u32();
        #[cfg(target_pointer_width = "64")]
        let value = self.to_u64();
        unsafe { mem::transmute::<_, Option<usize>>(value) }
    }

    /// Losslessly converts this `Num` to an `isize` value, if possible.
    #[inline]
    pub fn to_isize(&self) -> Option<isize> {
        #[cfg(target_pointer_width = "32")]
        let value = self.to_i32();
        #[cfg(target_pointer_width = "64")]
        let value = self.to_i64();
        unsafe { mem::transmute::<_, Option<isize>>(value) }
    }

    pub(crate) unsafe fn dealloc(&mut self) {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => (),
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => (),
            Value::F16_TYPE | Value::F32_TYPE | Value::F64_TYPE => (),
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

impl<'a> AsRef<Value<'a>> for Num<'a> {
    #[inline]
    fn as_ref(&self) -> &Value<'a> {
        self.as_value()
    }
}

impl<'a> AsMut<Value<'a>> for Num<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Value<'a> {
        self.as_mut_value()
    }
}

impl<'a> PartialEq for Num<'a> {
    fn eq(&self, that: &Num<'a>) -> bool {
        let self_tag = self.type_tag();
        let that_tag = that.type_tag();

        // Try unsigned comparison.
        let self_is_uint = self_tag == Value::U8_TYPE || self_tag == Value::U16_TYPE ||
            self_tag == Value::U32_TYPE || self_tag == Value::U64_TYPE;
        let that_is_uint = that_tag == Value::U8_TYPE || that_tag == Value::U16_TYPE ||
            that_tag == Value::U32_TYPE || that_tag == Value::U64_TYPE;
        if self_is_uint && that_is_uint {
            return self._1 == that._1;
        }

        // Try signed comparisons.
        let self_is_sint = self_tag == Value::I8_TYPE || self_tag == Value::I16_TYPE ||
            self_tag == Value::I32_TYPE || self_tag == Value::I64_TYPE;
        let that_is_sint = that_tag == Value::I8_TYPE || that_tag == Value::I16_TYPE ||
            that_tag == Value::I32_TYPE || that_tag == Value::I64_TYPE;
        if self_is_sint && that_is_sint {
            return self._1 as i64 == that._1 as i64;
        } else if self_is_uint && that_is_sint {
            return that._1 as i64 >= 0 && self._1 == that._1;
        } else if self_is_sint && that_is_uint {
            return self._1 as i64 >= 0 && self._1 == that._1;
        }

        // Try floating point comparisons.
        let self_is_f16 = self_tag == Value::F16_TYPE;
        let that_is_f16 = that_tag == Value::F16_TYPE;
        let self_is_f32 = self_tag == Value::F32_TYPE;
        let that_is_f32 = that_tag == Value::F32_TYPE;
        let self_is_f64 = self_tag == Value::F64_TYPE;
        let that_is_f64 = that_tag == Value::F64_TYPE;
        if (self_is_f32 || self_is_f64) && (that_is_f32 || that_is_f64) {
            let self_float = if self_is_f16 {
                f64::from(unsafe { mem::transmute::<u16, f16>(self._1 as u16) })
            } else if self_is_f32 {
                unsafe { mem::transmute::<u32, f32>(self._1 as u32) as f64 }
            } else {
                unsafe { mem::transmute::<u64, f64>(self._1) }
            };
            let that_float = if that_is_f16 {
                f64::from(unsafe { mem::transmute::<u16, f16>(that._1 as u16) })
            } else if that_is_f32 {
                unsafe { mem::transmute::<u32, f32>(that._1 as u32) as f64 }
            } else {
                unsafe { mem::transmute::<u64, f64>(that._1) }
            };
            return self_float == that_float || self_float.is_nan() && that_float.is_nan();
        } else if (self_is_f32 || self_is_f64) && (that_is_uint || that_is_sint) {
            let self_float = if self_is_f16 {
                f64::from(unsafe { mem::transmute::<u16, f16>(self._1 as u16) })
            } else if self_is_f32 {
                unsafe { mem::transmute::<u32, f32>(self._1 as u32) as f64 }
            } else {
                unsafe { mem::transmute::<u64, f64>(self._1) }
            };
            if that_is_uint {
                return self_float == that._1 as f64;
            } else if that_is_sint {
                return self_float == that._1 as i64 as f64;
            } else {
                unreachable!();
            }
        } else if (self_is_uint || self_is_sint) && (that_is_f32 || that_is_f64) {
            let that_float = if that_is_f16 {
                f64::from(unsafe { mem::transmute::<u16, f16>(that._1 as u16) })
            } else if that_is_f32 {
                unsafe { mem::transmute::<u32, f32>(that._1 as u32) as f64 }
            } else {
                unsafe { mem::transmute::<u64, f64>(that._1) }
            };
            if self_is_uint {
                return self._1 as f64 == that_float;
            } else if self_is_sint {
                return self._1 as i64 as f64 == that_float;
            } else {
                unreachable!();
            }
        }

        // Try big integer comparisons.
        let self_big_int = self_tag == Value::BIG_INT_TYPE;
        let that_big_int = that_tag == Value::BIG_INT_TYPE;
        if self_big_int && that_big_int {
            unimplemented!();
        } else if self_big_int {
            unimplemented!();
        } else if that_big_int {
            unimplemented!();
        }

        // Try big decimal comparisons.
        let self_big_dec = self_tag == Value::BIG_DEC_TYPE;
        let that_big_dec = that_tag == Value::BIG_DEC_TYPE;
        if self_big_dec && that_big_dec {
            unimplemented!();
        } else if self_big_dec {
            unimplemented!();
        } else if that_big_dec {
            unimplemented!();
        }

        unreachable!();
    }
}

impl<'a> PartialOrd for Num<'a> {
    fn partial_cmp(&self, that: &Num<'a>) -> Option<cmp::Ordering> {
        return Some(self.cmp(that))
    }
}

impl<'a> Ord for Num<'a> {
    fn cmp(&self, that: &Num<'a>) -> cmp::Ordering {
        let self_tag = self.type_tag();
        let that_tag = that.type_tag();

        // Try unsigned comparison.
        let self_is_uint = self_tag == Value::U8_TYPE || self_tag == Value::U16_TYPE ||
            self_tag == Value::U32_TYPE || self_tag == Value::U64_TYPE;
        let that_is_uint = that_tag == Value::U8_TYPE || that_tag == Value::U16_TYPE ||
            that_tag == Value::U32_TYPE || that_tag == Value::U64_TYPE;
        if self_is_uint && that_is_uint {
            return self._1.cmp(&that._1);
        }

        // Try signed comparisons.
        let self_is_sint = self_tag == Value::I8_TYPE || self_tag == Value::I16_TYPE ||
            self_tag == Value::I32_TYPE || self_tag == Value::I64_TYPE;
        let that_is_sint = that_tag == Value::I8_TYPE || that_tag == Value::I16_TYPE ||
            that_tag == Value::I32_TYPE || that_tag == Value::I64_TYPE;
        if self_is_sint && that_is_sint {
            return (self._1 as i64).cmp(&(that._1 as i64))
        } else if self_is_uint && that_is_sint {
            if that._1 as i64 >= 0 {
                return self._1.cmp(&that._1);
            } else {
                return cmp::Ordering::Greater;
            }
        } else if self_is_sint && that_is_uint {
            if self._1 as i64 >= 0 {
                return self._1.cmp(&that._1);
            } else {
                return cmp::Ordering::Less;
            }
        }

        // Try floating point comparisons.
        let self_is_f16 = self_tag == Value::F16_TYPE;
        let that_is_f16 = that_tag == Value::F16_TYPE;
        let self_is_f32 = self_tag == Value::F32_TYPE;
        let that_is_f32 = that_tag == Value::F32_TYPE;
        let self_is_f64 = self_tag == Value::F64_TYPE;
        let that_is_f64 = that_tag == Value::F64_TYPE;
        if (self_is_f16 || self_is_f32 || self_is_f64) && (that_is_f16 || that_is_f32 || that_is_f64) {
            let self_float = if self_is_f16 {
                f64::from(unsafe { mem::transmute::<u16, f16>(self._1 as u16) })
            } else if self_is_f32 {
                unsafe { mem::transmute::<u32, f32>(self._1 as u32) as f64 }
            } else {
                unsafe { mem::transmute::<u64, f64>(self._1) }
            };
            let that_float = if that_is_f16 {
                f64::from(unsafe { mem::transmute::<u16, f16>(that._1 as u16) })
            } else if that_is_f32 {
                unsafe { mem::transmute::<u32, f32>(that._1 as u32) as f64 }
            } else {
                unsafe { mem::transmute::<u64, f64>(that._1) }
            };
            if self_float.is_nan() && that_float.is_nan() {
                return cmp::Ordering::Equal;
            } else if self_float.is_nan() {
                return cmp::Ordering::Greater;
            } else if that_float.is_nan() {
                return cmp::Ordering::Less;
            } else {
                return self_float.partial_cmp(&that_float).unwrap();
            }
        } else if (self_is_f16 || self_is_f32 || self_is_f64) && (that_is_uint || that_is_sint) {
            let self_float = if self_is_f16 {
                f64::from(unsafe { mem::transmute::<u16, f16>(self._1 as u16) })
            } else if self_is_f32 {
                unsafe { mem::transmute::<u32, f32>(self._1 as u32) as f64 }
            } else {
                unsafe { mem::transmute::<u64, f64>(self._1) }
            };
            if self_float.is_nan() {
                return cmp::Ordering::Greater;
            } else if that_is_uint {
                return self_float.partial_cmp(&(that._1 as f64)).unwrap();
            } else if that_is_sint {
                return self_float.partial_cmp(&(that._1 as i64 as f64)).unwrap();
            } else {
                unreachable!();
            }
        } else if (self_is_uint || self_is_sint) && (that_is_f16 || that_is_f32 || that_is_f64) {
            let that_float = if that_is_f16 {
                f64::from(unsafe { mem::transmute::<u16, f16>(that._1 as u16) })
            } else if that_is_f32 {
                unsafe { mem::transmute::<u32, f32>(that._1 as u32) as f64 }
            } else {
                unsafe { mem::transmute::<u64, f64>(that._1) }
            };
            if that_float.is_nan() {
                return cmp::Ordering::Less;
            } else if self_is_uint {
                return (self._1 as f64).partial_cmp(&that_float).unwrap();
            } else if self_is_sint {
                return (self._1 as i64 as f64).partial_cmp(&that_float).unwrap();
            } else {
                unreachable!();
            }
        }

        // Try big integer comparisons.
        let self_big_int = self_tag == Value::BIG_INT_TYPE;
        let that_big_int = that_tag == Value::BIG_INT_TYPE;
        if self_big_int && that_big_int {
            unimplemented!();
        } else if self_big_int {
            unimplemented!();
        } else if that_big_int {
            unimplemented!();
        }

        // Try big decimal comparisons.
        let self_big_dec = self_tag == Value::BIG_DEC_TYPE;
        let that_big_dec = that_tag == Value::BIG_DEC_TYPE;
        if self_big_dec && that_big_dec {
            unimplemented!();
        } else if self_big_dec {
            unimplemented!();
        } else if that_big_dec {
            unimplemented!();
        }

        unreachable!();
    }
}

impl<'a> hash::Hash for Num<'a> {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                self._1.hash(hasher);
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                (self._1 as i64).hash(hasher);
            },
            Value::F16_TYPE => {
                let value = f64::from(unsafe { mem::transmute::<u16, f16>(self._1 as u16) });
                if value == value as u64 as f64 {
                    // Hash as u64, if equivalent.
                    (value as u64).hash(hasher);
                } else if value == value as i64 as f64 {
                    // Hash as i64, if equivalent.
                    (value as i64).hash(hasher);
                } else {
                    (self._1 as u16).hash(hasher);
                }
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) as f64 };
                if value == value as u64 as f64 {
                    // Hash as u64, if equivalent.
                    (value as u64).hash(hasher);
                } else if value == value as i64 as f64 {
                    // Hash as i64, if equivalent.
                    (value as i64).hash(hasher);
                } else {
                    (self._1 as u32).hash(hasher);
                }
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                if value == value as u64 as f64 {
                    // Hash as u64, if equivalent.
                    (value as u64).hash(hasher);
                } else if value == value as i64 as f64 {
                    // Hash as i64, if equivalent.
                    (value as i64).hash(hasher)
                } else {
                    self._1.hash(hasher);
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

impl<'a> fmt::Debug for Num<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.type_tag() {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE => {
                self._1.fmt(f)
            },
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE => {
                (self._1 as i64).fmt(f)
            },
            Value::F16_TYPE => {
                let value = unsafe { mem::transmute::<u16, f16>(self._1 as u16) };
                value.fmt(f)
            },
            Value::F32_TYPE => {
                let value = unsafe { mem::transmute::<u32, f32>(self._1 as u32) };
                value.fmt(f)
            },
            Value::F64_TYPE => {
                let value = unsafe { mem::transmute::<u64, f64>(self._1) };
                value.fmt(f)
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

impl<'a> Clone for Num<'a> {
    fn clone(&self) -> Num<'a> {
        let type_tag = self.type_tag();
        match type_tag {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE |
            Value::F16_TYPE | Value::F32_TYPE | Value::F64_TYPE => {
                Num {
                    _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(type_tag)) },
                    _1: self._1,
                    lifetime: PhantomData,
                }
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

impl<'a> TryClone for Num<'a> {
    fn try_clone(&self) -> Result<Num<'a>, HoldError> {
        let type_tag = self.type_tag();
        match type_tag {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE |
            Value::F16_TYPE | Value::F32_TYPE | Value::F64_TYPE => {
                Ok(Num {
                    _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(type_tag)) },
                    _1: self._1,
                    lifetime: PhantomData,
                })
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

impl<'a, 'b> CloneIntoHold<'a, Num<'a>> for Num<'b> {
    fn try_clone_into_hold(&self, _hold: &Hold<'a>) -> Result<Num<'a>, HoldError> {
        let type_tag = self.type_tag();
        match type_tag {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE |
            Value::F16_TYPE | Value::F32_TYPE | Value::F64_TYPE => {
                Ok(Num {
                    _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(type_tag)) },
                    _1: self._1,
                    lifetime: PhantomData,
                })
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

impl<'a, 'b> Stow<'b, Num<'b>> for Num<'a> {
    unsafe fn stow(src: *mut Num<'a>, dst: *mut Num<'b>, _hold: &Hold<'b>) -> Result<(), HoldError> {
        let type_tag = (*src).type_tag();
        match type_tag {
            Value::U8_TYPE | Value::U16_TYPE | Value::U32_TYPE | Value::U64_TYPE |
            Value::I8_TYPE | Value::I16_TYPE | Value::I32_TYPE | Value::I64_TYPE |
            Value::F16_TYPE | Value::F32_TYPE | Value::F64_TYPE => {
                ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(type_tag)));
                ptr::write(&mut (*dst)._1, (*src)._1);
            },
            Value::BIG_INT_TYPE => unimplemented!(),
            Value::BIG_DEC_TYPE => unimplemented!(),
            _ => unreachable!(),
        }
        Ok(())
    }

    unsafe fn unstow(_src: *mut Num<'a>, _dst: *mut Num<'b>) {
        unimplemented!();
    }
}

impl<'a> Drop for Num<'a> {
    fn drop(&mut self) {
        unsafe { self.dealloc(); }
    }
}

impl<'a> Default for Num<'a> {
    fn default() -> Num<'a> {
        Num::from(0)
    }
}

impl<'a> From<u8> for Num<'a> {
    fn from(value: u8) -> Num<'a> {
        Num::from_u8(value)
    }
}

impl<'a> From<i8> for Num<'a> {
    fn from(value: i8) -> Num<'a> {
        Num::from_i8(value)
    }
}

impl<'a> From<u16> for Num<'a> {
    fn from(value: u16) -> Num<'a> {
        Num::from_u16(value)
    }
}

impl<'a> From<i16> for Num<'a> {
    fn from(value: i16) -> Num<'a> {
        Num::from_i16(value)
    }
}

impl<'a> From<u32> for Num<'a> {
    fn from(value: u32) -> Num<'a> {
        Num::from_u32(value)
    }
}

impl<'a> From<i32> for Num<'a> {
    fn from(value: i32) -> Num<'a> {
        Num::from_i32(value)
    }
}

impl<'a> From<u64> for Num<'a> {
    fn from(value: u64) -> Num<'a> {
        Num::from_u64(value)
    }
}

impl<'a> From<i64> for Num<'a> {
    fn from(value: i64) -> Num<'a> {
        Num::from_i64(value)
    }
}

impl<'a> From<f16> for Num<'a> {
    fn from(value: f16) -> Num<'a> {
        Num::from_f16(value)
    }
}

impl<'a> From<f32> for Num<'a> {
    fn from(value: f32) -> Num<'a> {
        Num::from_f32(value)
    }
}

impl<'a> From<f64> for Num<'a> {
    fn from(value: f64) -> Num<'a> {
        Num::from_f64(value)
    }
}

impl<'a> From<usize> for Num<'a> {
    fn from(value: usize) -> Num<'a> {
        Num::from_usize(value)
    }
}

impl<'a> From<isize> for Num<'a> {
    fn from(value: isize) -> Num<'a> {
        Num::from_isize(value)
    }
}
