use core::cmp;
use core::fmt;
use core::hash;
use core::marker::PhantomData;
use core::mem;
use core::num::NonZeroU64;
use swim_core::f16;
use swim_mem::alloc::{Hold, HoldError, Stow, TryClone, CloneIntoHold};
use crate::item::{Item, Record, Data, Text, Num, Bool, Extant, Absent};

/// `Item` variant representing an unkeyed value.
#[derive(Eq, Ord)]
#[repr(C)]
pub struct Value<'a> {
    /// Discriminant.
    _0: NonZeroU64,
    // Reserved.
    _1: u64,
    /// Variant over allocation lifetime.
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Value<'a> {
    pub(crate) const ATTR_FLAG: u8 = 0x80;

    pub(crate) const TYPE_MASK: u8 = 0x7F;

    pub(crate) const ABSENT_TYPE: u8 = 0x01;
    pub(crate) const EXTANT_TYPE: u8 = 0x02;
    pub(crate) const FALSE_TYPE: u8 = 0x03;
    pub(crate) const TRUE_TYPE: u8 = 0x04;
    pub(crate) const U8_TYPE: u8 = 0x05;
    pub(crate) const U16_TYPE: u8 = 0x06;
    pub(crate) const U32_TYPE: u8 = 0x07;
    pub(crate) const U64_TYPE: u8 = 0x08;
    pub(crate) const I8_TYPE: u8 = 0x09;
    pub(crate) const I16_TYPE: u8 = 0x0A;
    pub(crate) const I32_TYPE: u8 = 0x0B;
    pub(crate) const I64_TYPE: u8 = 0x0C;
    pub(crate) const F16_TYPE: u8 = 0x0D;
    pub(crate) const F32_TYPE: u8 = 0x0E;
    pub(crate) const F64_TYPE: u8 = 0x0F;
    pub(crate) const BIG_INT_TYPE: u8 = 0x10;
    pub(crate) const BIG_DEC_TYPE: u8 = 0x11;
    pub(crate) const TEXT0_TYPE: u8 = 0x12;
    pub(crate) const TEXT7_TYPE: u8 = 0x19;
    pub(crate) const TEXT_TYPE: u8 = 0x1A;
    pub(crate) const DATA0_TYPE: u8 = 0x1B;
    pub(crate) const DATA7_TYPE: u8 = 0x22;
    pub(crate) const DATA_TYPE: u8 = 0x23;
    pub(crate) const RECORD0_TYPE: u8 = 0x24;
    pub(crate) const RECORD_TYPE: u8 = 0x25;

    pub(crate) const BOOL_TYPE_MIN: u8 = Value::FALSE_TYPE;
    pub(crate) const BOOL_TYPE_MAX: u8 = Value::TRUE_TYPE;
    pub(crate) const NUM_TYPE_MIN: u8 = Value::U8_TYPE;
    pub(crate) const NUM_TYPE_MAX: u8 = Value::BIG_DEC_TYPE;
    pub(crate) const TEXT_TYPE_MIN: u8 = Value::TEXT0_TYPE;
    pub(crate) const TEXT_TYPE_MAX: u8 = Value::TEXT_TYPE;
    pub(crate) const TEXT_EMBED_MAX: u8 = Value::TEXT7_TYPE - Value::TEXT0_TYPE;
    pub(crate) const DATA_TYPE_MIN: u8 = Value::DATA0_TYPE;
    pub(crate) const DATA_TYPE_MAX: u8 = Value::DATA_TYPE;
    pub(crate) const DATA_EMBED_MAX: u8 = Value::DATA7_TYPE - Value::DATA0_TYPE;
    pub(crate) const RECORD_TYPE_MIN: u8 = Value::RECORD0_TYPE;
    pub(crate) const RECORD_TYPE_MAX: u8 = Value::RECORD_TYPE;

    /// Returns a 64-bit discriminant with `tag` at the lowest byte address.
    #[cfg(target_endian = "big")]
    #[inline(always)]
    pub(crate) const fn discriminant(tag: u8) -> u64 {
        (tag as u64) << 56
    }
    /// Returns a 64-bit discriminant with `tag` at the lowest byte address.
    #[cfg(target_endian = "little")]
    #[inline(always)]
    pub(crate) const fn discriminant(tag: u8) -> u64 {
        tag as u64
    }

    pub fn try_hold_str(hold: &dyn Hold<'a>, data: &str) -> Result<Value<'a>, HoldError> {
        Ok(Text::try_hold_str(hold, data)?.into_value())
    }

    pub fn hold_str(hold: &dyn Hold<'a>, data: &str) -> Value<'a> {
        Value::try_hold_str(hold, data).unwrap()
    }

    pub fn from_str(data: &str) -> Value<'a> {
        Value::hold_str(Hold::global(), data)
    }

    /// Constructs a new `Value` from a `u8` value.
    pub const fn from_u8(value: u8) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::U8_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from an `i8` value.
    pub const fn from_i8(value: i8) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::I8_TYPE)) },
            _1: value as i64 as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from a `u16` value.
    pub const fn from_u16(value: u16) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::U16_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from an `i16` value.
    pub const fn from_i16(value: i16) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::I16_TYPE)) },
            _1: value as i64 as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from a `u32` value.
    pub const fn from_u32(value: u32) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::U32_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from an `i32` value.
    pub const fn from_i32(value: i32) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::I32_TYPE)) },
            _1: value as i64 as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from a `u64` value.
    pub const fn from_u64(value: u64) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::U64_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from an `i64` value.
    pub const fn from_i64(value: i64) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::I64_TYPE)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from an `f16` value.
    pub fn from_f16(value: f16) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::F16_TYPE)) },
            _1: unsafe { mem::transmute::<f16, u16>(value) as u64 },
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from an `f32` value.
    pub fn from_f32(value: f32) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::F32_TYPE)) },
            _1: unsafe { mem::transmute::<f32, u32>(value) as u64 },
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from an `f64` value.
    pub fn from_f64(value: f64) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::F64_TYPE)) },
            _1: unsafe { mem::transmute::<f64, u64>(value) },
            lifetime: PhantomData,
        }
    }

    /// Constructs a new `Value` from a `usize` value.
    #[cfg(target_pointer_width = "32")]
    pub const fn from_usize(value: usize) -> Value<'a> {
        Value::from_u32(value as u32)
    }
    /// Constructs a new `Value` from a `usize` value.
    #[cfg(target_pointer_width = "64")]
    pub const fn from_usize(value: usize) -> Value<'a> {
        Value::from_u64(value as u64)
    }

    /// Constructs a new `Value` from an `isize` value.
    #[cfg(target_pointer_width = "32")]
    pub const fn from_isize(value: isize) -> Value<'a> {
        Value::from_i32(value as i32)
    }
    /// Constructs a new `Value` from an `isize` value.
    #[cfg(target_pointer_width = "64")]
    pub const fn from_isize(value: isize) -> Value<'a> {
        Value::from_i64(value as i64)
    }

    /// Constructs a new `Value` from a `bool` value.
    pub const fn from_bool(value: bool) -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::FALSE_TYPE + value as u8)) },
            _1: value as u64,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new extant `Value`.
    pub const fn extant() -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::EXTANT_TYPE)) },
            _1: 0,
            lifetime: PhantomData,
        }
    }

    /// Constructs a new absent `Value`.
    pub const fn absent() -> Value<'a> {
        Value {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::ABSENT_TYPE)) },
            _1: 0,
            lifetime: PhantomData,
        }
    }

    /// Returns a pointer to the tag in the first byte of this `Value`.
    #[inline(always)]
    pub(crate) unsafe fn tag_ptr(&self) -> *mut u8 {
        mem::transmute::<&Value<'a>, *mut u8>(self)
    }

    /// Returns the tag from the first byte of this `Value`.
    #[inline(always)]
    pub(crate) fn tag(&self) -> u8 {
        unsafe { *self.tag_ptr() }
    }

    /// Returns the type tag from the low 7 bits of the first byte of this `Value`.
    #[inline(always)]
    pub(crate) fn type_tag(&self) -> u8 {
        self.tag() & Value::TYPE_MASK
    }

    /// Returns `true` if this `Value` is not `Absent`.
    pub fn is_defined(&self) -> bool {
        self.type_tag() > Value::ABSENT_TYPE
    }

    /// Returns `true` if this `Value` is a `Record`.
    pub fn is_record(&self) -> bool {
        let type_tag = self.type_tag();
        type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX
    }

    /// Returns `true` if this `Value` is `Data`.
    pub fn is_data(&self) -> bool {
        let type_tag = self.type_tag();
        type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX
    }

    /// Returns `true` if this `Value` is `Text`.
    pub fn is_text(&self) -> bool {
        let type_tag = self.type_tag();
        type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX
    }

    /// Returns `true` if this `Value` is a `Num`.
    pub fn is_num(&self) -> bool {
        let type_tag = self.type_tag();
        type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX
    }

    /// Returns `true` if this `Value` is a `Bool`.
    pub fn is_bool(&self) -> bool {
        let type_tag = self.type_tag();
        type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX
    }

    /// Returns `true` if this `Value` is `Extant`.
    pub fn is_extant(&self) -> bool {
        self.type_tag() == Value::EXTANT_TYPE
    }

    /// Returns `true` if this `Value` is `Absent`.
    pub fn is_absent(&self) -> bool {
        self.type_tag() == Value::ABSENT_TYPE
    }

    /// Downcasts this `Value` reference to a `Record` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Record`.
    #[inline]
    pub fn as_record(&self) -> &Record<'a> {
        if self.is_record() {
            unsafe { self.as_record_unchecked() }
        } else {
            panic!("not a Record");
        }
    }

    /// Downcasts this `Value` reference to a mutable `Record` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Record`.
    #[inline]
    pub fn as_mut_record(&mut self) -> &mut Record<'a> {
        if self.is_record() {
            unsafe { self.as_mut_record_unchecked() }
        } else {
            panic!("not a Record");
        }
    }

    /// Downcasts this `Value` reference to a `Data` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Data`.
    #[inline]
    pub fn as_data(&self) -> &Data<'a> {
        if self.is_data() {
            unsafe { self.as_data_unchecked() }
        } else {
            panic!("not Data");
        }
    }

    /// Downcasts this `Value` reference to a mutable `Data` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Data`.
    #[inline]
    pub fn as_mut_data(&mut self) -> &mut Data<'a> {
        if self.is_data() {
            unsafe { self.as_mut_data_unchecked() }
        } else {
            panic!("not Data");
        }
    }

    /// Downcasts this `Value` reference to a `Text` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Text`.
    #[inline]
    pub fn as_text(&self) -> &Text<'a> {
        if self.is_text() {
            unsafe { self.as_text_unchecked() }
        } else {
            panic!("not Text");
        }
    }

    /// Downcasts this `Value` reference to a mutable `Text` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Text`.
    #[inline]
    pub fn as_mut_text(&mut self) -> &mut Text<'a> {
        if self.is_text() {
            unsafe { self.as_mut_text_unchecked() }
        } else {
            panic!("not Text");
        }
    }

    /// Downcasts this `Value` reference to a `Num` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Num`.
    #[inline]
    pub fn as_num(&self) -> &Num<'a> {
        if self.is_num() {
            unsafe { self.as_num_unchecked() }
        } else {
            panic!("not a Num");
        }
    }

    /// Downcasts this `Value` reference to a mutable `Num` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Num`.
    #[inline]
    pub fn as_mut_num(&mut self) -> &mut Num<'a> {
        if self.is_num() {
            unsafe { self.as_mut_num_unchecked() }
        } else {
            panic!("not a Num");
        }
    }

    /// Downcasts this `Value` reference to a `Bool` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Bool`.
    #[inline]
    pub fn as_bool(&self) -> &Bool {
        if self.is_bool() {
            unsafe { self.as_bool_unchecked() }
        } else {
            panic!("not a Bool");
        }
    }

    /// Downcasts this `Value` reference to a mutable `Bool` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Bool`.
    #[inline]
    pub fn as_mut_bool(&mut self) -> &mut Bool {
        if self.is_bool() {
            unsafe { self.as_mut_bool_unchecked() }
        } else {
            panic!("not a Bool");
        }
    }

    /// Downcasts this `Value` reference to an `Extant` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Extant`.
    #[inline]
    pub fn as_extant(&self) -> &Extant {
        if self.is_extant() {
            unsafe { self.as_extant_unchecked() }
        } else {
            panic!("not Extant");
        }
    }

    /// Downcasts this `Value` reference to an `Absent` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Absent`.
    #[inline]
    pub fn as_absent(&self) -> &Absent {
        if self.is_absent() {
            unsafe { self.as_absent_unchecked() }
        } else {
            panic!("not Absent");
        }
    }

    /// Reinterprets this `Value` reference as a `Record` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Record`.
    #[inline]
    pub unsafe fn as_record_unchecked(&self) -> &Record<'a> {
        mem::transmute::<&Value<'a>, &Record<'a>>(self)
    }

    /// Reinterprets this `Value` reference as a mutable `Record` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Record`.
    #[inline]
    pub unsafe fn as_mut_record_unchecked(&mut self) -> &mut Record<'a> {
        mem::transmute::<&mut Value<'a>, &mut Record<'a>>(self)
    }

    /// Reinterprets this `Value` reference as a `Data` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Data`.
    #[inline]
    pub unsafe fn as_data_unchecked(&self) -> &Data<'a> {
        mem::transmute::<&Value<'a>, &Data<'a>>(self)
    }

    /// Reinterprets this `Value` reference as a mutable `Data` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Data`.
    #[inline]
    pub unsafe fn as_mut_data_unchecked(&mut self) -> &mut Data<'a> {
        mem::transmute::<&mut Value<'a>, &mut Data<'a>>(self)
    }

    /// Reinterprets this `Value` reference as a `Text` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Text`.
    #[inline]
    pub unsafe fn as_text_unchecked(&self) -> &Text<'a> {
        mem::transmute::<&Value<'a>, &Text<'a>>(self)
    }

    /// Reinterprets this `Value` reference as a `Text` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Text`.
    #[inline]
    pub unsafe fn as_mut_text_unchecked(&mut self) -> &mut Text<'a> {
        mem::transmute::<&mut Value<'a>, &mut Text<'a>>(self)
    }

    /// Reinterprets this `Value` reference as a `Num` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Num`.
    #[inline]
    pub unsafe fn as_num_unchecked(&self) -> &Num<'a> {
        mem::transmute::<&Value<'a>, &Num<'a>>(self)
    }

    /// Reinterprets this `Value` reference as a `Num` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Num`.
    #[inline]
    pub unsafe fn as_mut_num_unchecked(&mut self) -> &mut Num<'a> {
        mem::transmute::<&mut Value<'a>, &mut Num<'a>>(self)
    }

    /// Reinterprets this `Value` reference as a `Bool` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Bool`.
    #[inline]
    pub unsafe fn as_bool_unchecked(&self) -> &Bool {
        mem::transmute::<&Value<'a>, &Bool>(self)
    }

    /// Reinterprets this `Value` reference as a `Bool` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Bool`.
    #[inline]
    pub unsafe fn as_mut_bool_unchecked(&mut self) -> &mut Bool {
        mem::transmute::<&mut Value<'a>, &mut Bool>(self)
    }

    /// Reinterprets this `Value` reference as an `Extant` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Extant`.
    #[inline]
    pub unsafe fn as_extant_unchecked(&self) -> &Extant {
        mem::transmute::<&Value<'a>, &Extant>(self)
    }

    /// Reinterprets this `Value` reference as an `Extant` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Extant`.
    #[inline]
    pub unsafe fn as_mut_extant_unchecked(&mut self) -> &mut Extant {
        mem::transmute::<&mut Value<'a>, &mut Extant>(self)
    }

    /// Reinterprets this `Value` reference as an `Absent` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Absent`.
    #[inline]
    pub unsafe fn as_absent_unchecked(&self) -> &Absent {
        mem::transmute::<&Value<'a>, &Absent>(self)
    }

    /// Reinterprets this `Value` reference as an `Absent` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Absent`.
    #[inline]
    pub unsafe fn as_mut_absent_unchecked(&mut self) -> &mut Absent {
        mem::transmute::<&mut Value<'a>, &mut Absent>(self)
    }

    /// Downcasts this `Value` to a `Record`.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Record`.
    #[inline]
    pub fn into_record(self) -> Record<'a> {
        if self.is_record() {
            unsafe { self.into_record_unchecked() }
        } else {
            panic!("not a Record");
        }
    }

    /// Downcasts this `Value` to `Data`.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Data`.
    #[inline]
    pub fn into_data(self) -> Data<'a> {
        if self.is_data() {
            unsafe { self.into_data_unchecked() }
        } else {
            panic!("not Data");
        }
    }

    /// Downcasts this `Value` to `Text`.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Text`.
    #[inline]
    pub fn into_text(self) -> Text<'a> {
        if self.is_text() {
            unsafe { self.into_text_unchecked() }
        } else {
            panic!("not Text");
        }
    }

    /// Downcasts this `Value` to a `Num`.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Num`.
    #[inline]
    pub fn into_num(self) -> Num<'a> {
        if self.is_num() {
            unsafe { self.into_num_unchecked() }
        } else {
            panic!("not a Num");
        }
    }

    /// Downcasts this `Value` to a `Bool`.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, a `Bool`.
    #[inline]
    pub fn into_bool(self) -> Bool {
        if self.is_bool() {
            unsafe { self.into_bool_unchecked() }
        } else {
            panic!("not a Bool");
        }
    }

    /// Downcasts this `Value` to `Extant`.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Extant`.
    #[inline]
    pub fn into_extant(self) -> Extant {
        if self.is_extant() {
            unsafe { self.into_extant_unchecked() }
        } else {
            panic!("not Extant");
        }
    }

    /// Downcasts this `Value` to `Absent`.
    ///
    /// # Panics
    ///
    /// Panics if this `Value` is not, in fact, `Absent`.
    #[inline]
    pub fn into_absent(self) -> Absent {
        if self.is_absent() {
            unsafe { self.into_absent_unchecked() }
        } else {
            panic!("not Absent");
        }
    }

    /// Reinterprets this `Value` as a `Record`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Record`.
    #[inline]
    pub unsafe fn into_record_unchecked(self) -> Record<'a> {
        mem::transmute::<Value<'a>, Record<'a>>(self)
    }

    /// Reinterprets this `Value` as `Data`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Data`.
    #[inline]
    pub unsafe fn into_data_unchecked(self) -> Data<'a> {
        mem::transmute::<Value<'a>, Data<'a>>(self)
    }

    /// Reinterprets this `Value` as `Text`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Text`.
    #[inline]
    pub unsafe fn into_text_unchecked(self) -> Text<'a> {
        mem::transmute::<Value<'a>, Text<'a>>(self)
    }

    /// Reinterprets this `Value` as a `Num`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Num`.
    #[inline]
    pub unsafe fn into_num_unchecked(self) -> Num<'a> {
        mem::transmute::<Value<'a>, Num<'a>>(self)
    }

    /// Reinterprets this `Value` as a `Bool`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Bool`.
    #[inline]
    pub unsafe fn into_bool_unchecked(self) -> Bool {
        mem::transmute::<Value<'a>, Bool>(self)
    }

    /// Reinterprets this `Value` as `Extant`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Extant`.
    #[inline]
    pub unsafe fn into_extant_unchecked(self) -> Extant {
        mem::transmute::<Value<'a>, Extant>(self)
    }

    /// Reinterprets this `Value` as `Absent`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Absent`.
    #[inline]
    pub unsafe fn into_absent_unchecked(self) -> Absent {
        mem::transmute::<Value<'a>, Absent>(self)
    }

    /// Upcasts this `Value` to an `Item`.
    #[inline]
    pub fn into_item(self) -> Item<'a> {
        Item::from_value(self)
    }

    /// Downcasts this `Value` reference to a typechecked `Record` reference.
    #[inline]
    pub fn cast_as_record(&self) -> Option<&Record<'a>> {
        if self.is_record() {
            Some(unsafe { self.as_record_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked, mutable `Record` reference.
    #[inline]
    pub fn cast_as_mut_record(&mut self) -> Option<&mut Record<'a>> {
        if self.is_record() {
            Some(unsafe { self.as_mut_record_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked `Data` reference.
    #[inline]
    pub fn cast_as_data(&self) -> Option<&Data<'a>> {
        if self.is_data() {
            Some(unsafe { self.as_data_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked, mutable `Data` reference.
    #[inline]
    pub fn cast_as_mut_data(&mut self) -> Option<&mut Data<'a>> {
        if self.is_data() {
            Some(unsafe { self.as_mut_data_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked `Text` reference.
    #[inline]
    pub fn cast_as_text(&self) -> Option<&Text<'a>> {
        if self.is_text() {
            Some(unsafe { self.as_text_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked, mutable `Text` reference.
    #[inline]
    pub fn cast_as_mut_text(&mut self) -> Option<&mut Text<'a>> {
        if self.is_text() {
            Some(unsafe { self.as_mut_text_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked `Num` reference.
    #[inline]
    pub fn cast_as_num(&self) -> Option<&Num<'a>> {
        if self.is_num() {
            Some(unsafe { self.as_num_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked, mutable `Num` reference.
    #[inline]
    pub fn cast_as_mut_num(&mut self) -> Option<&mut Num<'a>> {
        if self.is_num() {
            Some(unsafe { self.as_mut_num_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked `Bool` reference.
    #[inline]
    pub fn cast_as_bool(&self) -> Option<&Bool> {
        if self.is_bool() {
            Some(unsafe { self.as_bool_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked, mutable `Bool` reference.
    #[inline]
    pub fn cast_as_mut_bool(&mut self) -> Option<&mut Bool> {
        if self.is_bool() {
            Some(unsafe { self.as_mut_bool_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked `Extant` reference.
    #[inline]
    pub fn cast_as_extant(&self) -> Option<&Extant> {
        if self.is_extant() {
            Some(unsafe { self.as_extant_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` reference to a typechecked `Absent` reference.
    #[inline]
    pub fn cast_as_absent(&self) -> Option<&Absent> {
        if self.is_absent() {
            Some(unsafe { self.as_absent_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` to a typechecked `Record`.
    #[inline]
    pub fn cast_into_record(self) -> Option<Record<'a>> {
        if self.is_record() {
            Some(unsafe { self.into_record_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` to typechecked `Data`.
    #[inline]
    pub fn cast_into_data(self) -> Option<Data<'a>> {
        if self.is_data() {
            Some(unsafe { self.into_data_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` to typechecked `Text`.
    #[inline]
    pub fn cast_into_text(self) -> Option<Text<'a>> {
        if self.is_text() {
            Some(unsafe { self.into_text_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` to a typechecked `Num`.
    #[inline]
    pub fn cast_into_num(self) -> Option<Num<'a>> {
        if self.is_num() {
            Some(unsafe { self.into_num_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` to a typechecked `Bool`.
    #[inline]
    pub fn cast_into_bool(self) -> Option<Bool> {
        if self.is_bool() {
            Some(unsafe { self.into_bool_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` to typechecked `Extant`.
    #[inline]
    pub fn cast_into_extant(self) -> Option<Extant> {
        if self.is_extant() {
            Some(unsafe { self.into_extant_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Value` to typechecked `Absent`.
    #[inline]
    pub fn cast_into_absent(self) -> Option<Absent> {
        if self.is_absent() {
            Some(unsafe { self.into_absent_unchecked() })
        } else {
            None
        }
    }
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, that: &Value<'a>) -> bool {
        unsafe {
            let this_type_tag = self.type_tag();
            let that_type_tag = that.type_tag();
            if this_type_tag >= Value::RECORD_TYPE_MIN && this_type_tag <= Value::RECORD_TYPE_MAX {
                that_type_tag >= Value::RECORD_TYPE_MIN && that_type_tag <= Value::RECORD_TYPE_MAX
                    && self.as_record_unchecked().eq(that.as_record_unchecked())
            } else if this_type_tag >= Value::DATA_TYPE_MIN && this_type_tag <= Value::DATA_TYPE_MAX {
                that_type_tag >= Value::DATA_TYPE_MIN && that_type_tag <= Value::DATA_TYPE_MAX
                    && self.as_data_unchecked().eq(that.as_data_unchecked())
            } else if this_type_tag >= Value::TEXT_TYPE_MIN && this_type_tag <= Value::TEXT_TYPE_MAX {
                that_type_tag >= Value::TEXT_TYPE_MIN && that_type_tag <= Value::TEXT_TYPE_MAX
                    && self.as_text_unchecked().eq(that.as_text_unchecked())
            } else if this_type_tag >= Value::NUM_TYPE_MIN && this_type_tag <= Value::NUM_TYPE_MAX {
                that_type_tag >= Value::NUM_TYPE_MIN && that_type_tag <= Value::NUM_TYPE_MAX
                    && self.as_num_unchecked().eq(that.as_num_unchecked())
            } else if this_type_tag >= Value::BOOL_TYPE_MIN && this_type_tag <= Value::BOOL_TYPE_MAX {
                that_type_tag >= Value::BOOL_TYPE_MIN && that_type_tag <= Value::BOOL_TYPE_MAX
                    && self.as_bool_unchecked().eq(that.as_bool_unchecked())
            } else if this_type_tag == Value::EXTANT_TYPE {
                that_type_tag == Value::EXTANT_TYPE
                    && self.as_extant_unchecked().eq(that.as_extant_unchecked())
            } else if this_type_tag == Value::ABSENT_TYPE {
                this_type_tag == Value::ABSENT_TYPE
                    && self.as_absent_unchecked().eq(that.as_absent_unchecked())
            } else {
                false
            }
        }
    }

    fn ne(&self, that: &Value<'a>) -> bool {
        unsafe {
            let this_type_tag = self.type_tag();
            let that_type_tag = that.type_tag();
            if this_type_tag >= Value::RECORD_TYPE_MIN && this_type_tag <= Value::RECORD_TYPE_MAX {
                that_type_tag < Value::RECORD_TYPE_MIN || that_type_tag > Value::RECORD_TYPE_MAX
                    || self.as_record_unchecked().ne(that.as_record_unchecked())
            } else if this_type_tag >= Value::DATA_TYPE_MIN && this_type_tag <= Value::DATA_TYPE_MAX {
                that_type_tag < Value::DATA_TYPE_MIN || that_type_tag > Value::DATA_TYPE_MAX
                    || self.as_data_unchecked().ne(that.as_data_unchecked())
            } else if this_type_tag >= Value::TEXT_TYPE_MIN && this_type_tag <= Value::TEXT_TYPE_MAX {
                that_type_tag < Value::TEXT_TYPE_MIN || that_type_tag > Value::TEXT_TYPE_MAX
                    || self.as_text_unchecked().ne(that.as_text_unchecked())
            } else if this_type_tag >= Value::NUM_TYPE_MIN && this_type_tag <= Value::NUM_TYPE_MAX {
                that_type_tag < Value::NUM_TYPE_MIN || that_type_tag > Value::NUM_TYPE_MAX
                    || self.as_num_unchecked().ne(that.as_num_unchecked())
            } else if this_type_tag >= Value::BOOL_TYPE_MIN && this_type_tag <= Value::BOOL_TYPE_MAX {
                that_type_tag < Value::BOOL_TYPE_MIN || that_type_tag > Value::BOOL_TYPE_MAX
                    || self.as_bool_unchecked().ne(that.as_bool_unchecked())
            } else if this_type_tag == Value::EXTANT_TYPE {
                that_type_tag != Value::EXTANT_TYPE
                    || self.as_extant_unchecked().ne(that.as_extant_unchecked())
            } else if this_type_tag == Value::ABSENT_TYPE {
                this_type_tag != Value::ABSENT_TYPE
                    || self.as_absent_unchecked().ne(that.as_absent_unchecked())
            } else {
                true
            }
        }
    }
}

impl<'a> cmp::PartialOrd<Value<'a>> for Value<'a> {
    fn partial_cmp(&self, _that: &Value<'a>) -> Option<cmp::Ordering> {
        unimplemented!(); // TODO
    }

    fn lt(&self, _that: &Value<'a>) -> bool {
        unimplemented!(); // TODO
    }

    fn le(&self, _that: &Value<'a>) -> bool {
        unimplemented!(); // TODO
    }

    fn ge(&self, _that: &Value<'a>) -> bool {
        unimplemented!(); // TODO
    }

    fn gt(&self, _that: &Value<'a>) -> bool {
        unimplemented!(); // TODO
    }
}

impl<'a> hash::Hash for Value<'a> {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX {
                self.as_record_unchecked().hash(hasher);
            } else if type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX {
                self.as_data_unchecked().hash(hasher);
            } else if type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX {
                self.as_text_unchecked().hash(hasher);
            } else if type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX {
                self.as_num_unchecked().hash(hasher);
            } else if type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX {
                self.as_bool_unchecked().hash(hasher);
            } else if type_tag == Value::EXTANT_TYPE {
                self.as_extant_unchecked().hash(hasher);
            } else if type_tag == Value::ABSENT_TYPE {
                self.as_absent_unchecked().hash(hasher);
            } else {
                unreachable!();
            }
        }
    }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX {
                fmt::Debug::fmt(self.as_record_unchecked(), f)
            } else if type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX {
                fmt::Debug::fmt(self.as_data_unchecked(), f)
            } else if type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX {
                fmt::Debug::fmt(self.as_text_unchecked(), f)
            } else if type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX {
                fmt::Debug::fmt(self.as_num_unchecked(), f)
            } else if type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX {
                fmt::Debug::fmt(self.as_bool_unchecked(), f)
            } else if type_tag == Value::EXTANT_TYPE {
                fmt::Debug::fmt(self.as_extant_unchecked(), f)
            } else if type_tag == Value::ABSENT_TYPE {
                fmt::Debug::fmt(self.as_absent_unchecked(), f)
            } else {
                unreachable!();
            }
        }
    }
}

impl<'a> Clone for Value<'a> {
    fn clone(&self) -> Value<'a> {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX {
                self.as_record_unchecked().clone().into_value()
            } else if type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX {
                self.as_data_unchecked().clone().into_value()
            } else if type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX {
                self.as_text_unchecked().clone().into_value()
            } else if type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX {
                self.as_num_unchecked().clone().into_value()
            } else if type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX {
                self.as_bool_unchecked().clone().into_value()
            } else if type_tag == Value::EXTANT_TYPE {
                self.as_extant_unchecked().clone().into_value()
            } else if type_tag == Value::ABSENT_TYPE {
                self.as_absent_unchecked().clone().into_value()
            } else {
                unreachable!();
            }
        }
    }
}

impl<'a> TryClone for Value<'a> {
    fn try_clone(&self) -> Result<Value<'a>, HoldError> {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX {
                Ok(self.as_record_unchecked().try_clone()?.into_value())
            } else if type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX {
                Ok(self.as_data_unchecked().try_clone()?.into_value())
            } else if type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX {
                Ok(self.as_text_unchecked().try_clone()?.into_value())
            } else if type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX {
                Ok(self.as_num_unchecked().try_clone()?.into_value())
            } else if type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX {
                Ok(self.as_bool_unchecked().try_clone()?.into_value())
            } else if type_tag == Value::EXTANT_TYPE {
                Ok(self.as_extant_unchecked().try_clone()?.into_value())
            } else if type_tag == Value::ABSENT_TYPE {
                Ok(self.as_absent_unchecked().try_clone()?.into_value())
            } else {
                unreachable!();
            }
        }
    }
}

impl<'a, 'b> CloneIntoHold<'a, Value<'a>> for Value<'b> {
    fn try_clone_into_hold(&self, hold: &Hold<'a>) -> Result<Value<'a>, HoldError> {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX {
                Ok(<Record<'b> as CloneIntoHold<'a, Record<'a>>>::try_clone_into_hold(self.as_record_unchecked(), hold)?.into_value())
            } else if type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX {
                Ok(<Data<'b> as CloneIntoHold<'a, Data<'a>>>::try_clone_into_hold(self.as_data_unchecked(), hold)?.into_value())
            } else if type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX {
                Ok(<Text<'b> as CloneIntoHold<'a, Text<'a>>>::try_clone_into_hold(self.as_text_unchecked(), hold)?.into_value())
            } else if type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX {
                Ok(<Num<'b> as CloneIntoHold<'a, Num<'a>>>::try_clone_into_hold(self.as_num_unchecked(), hold)?.into_value())
            } else if type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX {
                Ok(<Bool as CloneIntoHold<'a, Bool>>::try_clone_into_hold(self.as_bool_unchecked(), hold)?.into_value())
            } else if type_tag == Value::EXTANT_TYPE {
                Ok(<Extant as CloneIntoHold<'a, Extant>>::try_clone_into_hold(self.as_extant_unchecked(), hold)?.into_value())
            } else if type_tag == Value::ABSENT_TYPE {
                Ok(<Absent as CloneIntoHold<'a, Absent>>::try_clone_into_hold(self.as_absent_unchecked(), hold)?.into_value())
            } else {
                unreachable!();
            }
        }
    }
}

impl<'a, 'b> Stow<'b, Value<'b>> for Value<'a> {
    unsafe fn stow(src: *mut Value<'a>, dst: *mut Value<'b>, hold: &Hold<'b>) -> Result<(), HoldError> {
        let type_tag = (*src).type_tag();
        if type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX {
            Stow::stow(&mut *(*src).as_mut_record_unchecked(), dst as *mut Record<'b>, hold)
        } else if type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX {
            Stow::stow(&mut *(*src).as_mut_data_unchecked(), dst as *mut Data<'b>, hold)
        } else if type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX {
            Stow::stow(&mut *(*src).as_mut_text_unchecked(), dst as *mut Text<'b>, hold)
        } else if type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX {
            Stow::stow(&mut *(*src).as_mut_num_unchecked(), dst as *mut Num<'b>, hold)
        } else if type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX {
            Stow::stow(&mut *(*src).as_mut_bool_unchecked(), dst as *mut Bool, hold)
        } else if type_tag == Value::EXTANT_TYPE {
            Stow::stow(&mut *(*src).as_mut_extant_unchecked(), dst as *mut Extant, hold)
        } else if type_tag == Value::ABSENT_TYPE {
            Stow::stow(&mut *(*src).as_mut_absent_unchecked(), dst as *mut Absent, hold)
        } else {
            unreachable!();
        }
    }

    unsafe fn unstow(src: *mut Value<'a>, dst: *mut Value<'b>) {
        let type_tag = (*src).type_tag();
        if type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX {
            Stow::unstow(&mut *(*src).as_mut_record_unchecked(), dst as *mut Record<'b>)
        } else if type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX {
            Stow::unstow(&mut *(*src).as_mut_data_unchecked(), dst as *mut Data<'b>)
        } else if type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX {
            Stow::unstow(&mut *(*src).as_mut_text_unchecked(), dst as *mut Text<'b>)
        } else if type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX {
            Stow::unstow(&mut *(*src).as_mut_num_unchecked(), dst as *mut Num<'b>)
        } else if type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX {
            Stow::unstow(&mut *(*src).as_mut_bool_unchecked(), dst as *mut Bool)
        } else if type_tag == Value::EXTANT_TYPE {
            Stow::unstow(&mut *(*src).as_mut_extant_unchecked(), dst as *mut Extant)
        } else if type_tag == Value::ABSENT_TYPE {
            Stow::unstow(&mut *(*src).as_mut_absent_unchecked(), dst as *mut Absent)
        } else {
            unreachable!();
        }
    }
}

impl<'a> Drop for Value<'a> {
    fn drop(&mut self) {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag >= Value::RECORD_TYPE_MIN && type_tag <= Value::RECORD_TYPE_MAX {
                self.as_mut_record_unchecked().dealloc();
            } else if type_tag >= Value::DATA_TYPE_MIN && type_tag <= Value::DATA_TYPE_MAX {
                self.as_mut_data_unchecked().dealloc();
            } else if type_tag >= Value::TEXT_TYPE_MIN && type_tag <= Value::TEXT_TYPE_MAX {
                self.as_mut_text_unchecked().dealloc();
            } else if type_tag >= Value::NUM_TYPE_MIN && type_tag <= Value::NUM_TYPE_MAX {
                self.as_mut_num_unchecked().dealloc();
            } else if type_tag >= Value::BOOL_TYPE_MIN && type_tag <= Value::BOOL_TYPE_MAX {
                // nop
            } else if type_tag == Value::EXTANT_TYPE {
                // nop
            } else if type_tag == Value::ABSENT_TYPE {
                // nop
            } else {
                unreachable!();
            }
        }
    }
}

impl<'a> Default for Value<'a> {
    #[inline]
    fn default() -> Value<'a> {
        Value::absent()
    }
}

impl<'a> From<Record<'a>> for Value<'a> {
    #[inline]
    fn from(record: Record<'a>) -> Value<'a> {
        record.into_value()
    }
}

impl<'a> From<Data<'a>> for Value<'a> {
    #[inline]
    fn from(data: Data<'a>) -> Value<'a> {
        data.into_value()
    }
}

impl<'a> From<Text<'a>> for Value<'a> {
    #[inline]
    fn from(text: Text<'a>) -> Value<'a> {
        text.into_value()
    }
}

impl<'a> From<Num<'a>> for Value<'a> {
    #[inline]
    fn from(num: Num<'a>) -> Value<'a> {
        num.into_value()
    }
}

impl<'a> From<Bool> for Value<'a> {
    #[inline]
    fn from(bool: Bool) -> Value<'a> {
        bool.into_value()
    }
}

impl<'a> From<Extant> for Value<'a> {
    #[inline]
    fn from(extant: Extant) -> Value<'a> {
        extant.into_value()
    }
}

impl<'a> From<Absent> for Value<'a> {
    #[inline]
    fn from(absent: Absent) -> Value<'a> {
        absent.into_value()
    }
}

impl<'a> From<u8> for Value<'a> {
    fn from(value: u8) -> Value<'a> {
        Value::from_u8(value)
    }
}

impl<'a> From<i8> for Value<'a> {
    fn from(value: i8) -> Value<'a> {
        Value::from_i8(value)
    }
}

impl<'a> From<u16> for Value<'a> {
    fn from(value: u16) -> Value<'a> {
        Value::from_u16(value)
    }
}

impl<'a> From<i16> for Value<'a> {
    fn from(value: i16) -> Value<'a> {
        Value::from_i16(value)
    }
}

impl<'a> From<u32> for Value<'a> {
    fn from(value: u32) -> Value<'a> {
        Value::from_u32(value)
    }
}

impl<'a> From<i32> for Value<'a> {
    fn from(value: i32) -> Value<'a> {
        Value::from_i32(value)
    }
}

impl<'a> From<u64> for Value<'a> {
    fn from(value: u64) -> Value<'a> {
        Value::from_u64(value)
    }
}

impl<'a> From<i64> for Value<'a> {
    fn from(value: i64) -> Value<'a> {
        Value::from_i64(value)
    }
}

impl<'a> From<f16> for Value<'a> {
    fn from(value: f16) -> Value<'a> {
        Value::from_f16(value)
    }
}

impl<'a> From<f32> for Value<'a> {
    fn from(value: f32) -> Value<'a> {
        Value::from_f32(value)
    }
}

impl<'a> From<f64> for Value<'a> {
    fn from(value: f64) -> Value<'a> {
        Value::from_f64(value)
    }
}

impl<'a> From<usize> for Value<'a> {
    fn from(value: usize) -> Value<'a> {
        Value::from_usize(value)
    }
}

impl<'a> From<isize> for Value<'a> {
    fn from(value: isize) -> Value<'a> {
        Value::from_isize(value)
    }
}

impl<'a> From<bool> for Value<'a> {
    #[inline]
    fn from(value: bool) -> Value<'a> {
        Value::from_bool(value)
    }
}
