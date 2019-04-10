use core::cmp;
use core::fmt;
use core::hash;
use core::mem;
use core::ptr;
use swim_core::f16;
use swim_mem::alloc::{Hold, HoldError, Stow, TryClone, CloneIntoHold};
use crate::item::{Field, Attr, Slot, Value, Record, Data, Text, Num, Bool, Extant, Absent};

/// Union of `Field` and `Value` representing a member of a `Record`.
#[derive(Eq, Ord)]
#[repr(C)]
pub struct Item<'a> {
    val: Value<'a>,
    key: Option<Value<'a>>,
}

impl<'a> Item<'a> {
    pub fn attr(key: Text<'a>, val: Value<'a>) -> Item<'a> {
        Attr::new(key, val).into_item()
    }

    pub fn slot(key: Value<'a>, val: Value<'a>) -> Item<'a> {
        Slot::new(key, val).into_item()
    }

    /// Constructs a new `Item` from a `u8` value.
    pub const fn from_u8(value: u8) -> Item<'a> {
        Item {
            val: Value::from_u8(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from an `i8` value.
    pub const fn from_i8(value: i8) -> Item<'a> {
        Item {
            val: Value::from_i8(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from a `u16` value.
    pub const fn from_u16(value: u16) -> Item<'a> {
        Item {
            val: Value::from_u16(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from an `i16` value.
    pub const fn from_i16(value: i16) -> Item<'a> {
        Item {
            val: Value::from_i16(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from a `u32` value.
    pub const fn from_u32(value: u32) -> Item<'a> {
        Item {
            val: Value::from_u32(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from an `i32` value.
    pub const fn from_i32(value: i32) -> Item<'a> {
        Item {
            val: Value::from_i32(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from a `u64` value.
    pub const fn from_u64(value: u64) -> Item<'a> {
        Item {
            val: Value::from_u64(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from an `i64` value.
    pub const fn from_i64(value: i64) -> Item<'a> {
        Item {
            val: Value::from_i64(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from an `f16` value.
    pub fn from_f16(value: f16) -> Item<'a> {
        Item {
            val: Value::from_f16(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from an `f32` value.
    pub fn from_f32(value: f32) -> Item<'a> {
        Item {
            val: Value::from_f32(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from an `f64` value.
    pub fn from_f64(value: f64) -> Item<'a> {
        Item {
            val: Value::from_f64(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from a `usize` value.
    pub const fn from_usize(value: usize) -> Item<'a> {
        Item {
            val: Value::from_usize(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from an `isize` value.
    pub const fn from_isize(value: isize) -> Item<'a> {
        Item {
            val: Value::from_isize(value),
            key: None,
        }
    }

    /// Constructs a new `Item` from a `bool` value.
    pub const fn from_bool(value: bool) -> Item<'a> {
        Item {
            val: Value::from_bool(value),
            key: None,
        }
    }

    /// Constructs a new extant `Item`.
    pub const fn extant() -> Item<'a> {
        Item {
            val: Value::extant(),
            key: None,
        }
    }

    /// Constructs a new absent `Item`.
    pub const fn absent() -> Item<'a> {
        Item {
            val: Value::absent(),
            key: None,
        }
    }

    #[inline]
    pub(crate) const fn from_value(value: Value<'a>) -> Item<'a> {
        Item {
            val: value,
            key: None,
        }
    }

    /// Returns a reference to this `Item`'s optional key.
    #[inline]
    pub fn get_key(&self) -> &Option<Value<'a>> {
        &self.key
    }

    /// Returns a mutable reference to this `Item`'s optional key.
    #[inline]
    pub fn get_key_mut(&mut self) -> &mut Option<Value<'a>> {
        &mut self.key
    }

    /// Returns a reference to this `Item`'s key.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item`'s key is defined.
    #[inline]
    pub unsafe fn get_key_unchecked(&self) -> &Value<'a> {
        mem::transmute::<&Option<Value<'a>>, &Value<'a>>(&self.key)
    }

    /// Returns a mutable reference to this `Item`'s key.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item`'s key is defined.
    #[inline]
    pub unsafe fn get_key_unchecked_mut(&mut self) -> &mut Value<'a> {
        mem::transmute::<&mut Option<Value<'a>>, &mut Value<'a>>(&mut self.key)
    }

    /// Returns a reference to this `Item`'s value.
    #[inline]
    pub fn get_val(&self) -> &Value<'a> {
        &self.val
    }

    /// Returns a mutable reference to this `Item`'s value.
    #[inline]
    pub fn get_val_mut(&mut self) -> &mut Value<'a> {
        &mut self.val
    }

    /// Returns a pair of references to this `Item`'s optional key and value.
    #[inline]
    pub fn get_key_val(&self) -> (&Option<Value<'a>>, &Value<'a>) {
        (&self.key, &self.val)
    }

    /// Returns a pair of mutable references to this `Item`'s optional key and value.
    #[inline]
    pub fn get_key_val_mut(&mut self) -> (&mut Option<Value<'a>>, &mut Value<'a>) {
        (&mut self.key, &mut self.val)
    }

    /// Returns this `Item`'s key, dropping its value.
    #[inline]
    pub fn into_key(mut self) -> Option<Value<'a>> {
        let key = unsafe { ptr::read(&self.key) };
        unsafe { ptr::drop_in_place(&mut self.val); }
        mem::forget(self);
        key
    }

    /// Returns this `Item`'s value, dropping its optional key.
    #[inline]
    pub fn into_val(mut self) -> Value<'a> {
        unsafe { ptr::drop_in_place(&mut self.key) };
        let val = unsafe { ptr::read(&self.val) };
        mem::forget(self);
        val
    }

    /// Returns this `Item`'s optional key and value as a pair.
    #[inline]
    pub fn into_key_val(self) -> (Option<Value<'a>>, Value<'a>) {
        let key = unsafe { ptr::read(&self.key) };
        let val = unsafe { ptr::read(&self.val) };
        mem::forget(self);
        (key, val)
    }

    /// Returnsa pointer to the tag in the first byte of this `Item`'s key.
    #[inline(always)]
    pub(crate) fn key_tag_ptr(&self) -> *mut u8 {
        unsafe { self.get_key_unchecked().tag_ptr() }
    }

    /// Returns the tag from the first byte of this `Item`'s key.
    #[inline(always)]
    pub(crate) fn key_tag(&self) -> u8 {
        unsafe { self.get_key_unchecked() }.tag()
    }

    /// Returns the tag from the first byte of this `Item`'s value.
    #[inline(always)]
    pub(crate) fn val_tag(&self) -> u8 {
        self.val.tag()
    }

    /// Returns `true` if this `Item` is not `Absent`.
    pub fn is_defined(&self) -> bool {
        self.key_tag() != 0 || self.val_tag() != 0
    }

    /// Returns `true` if this `Item` is a `Field`.
    pub fn is_field(&self) -> bool {
        self.key_tag() != 0
    }

    /// Returns `true` if this `Item` is an `Attr`.
    pub fn is_attr(&self) -> bool {
        self.key_tag() & Value::ATTR_FLAG != 0
    }

    /// Returns `true` if this `Item` is a `Slot`.
    pub fn is_slot(&self) -> bool {
        let key_tag = self.key_tag();
        key_tag ^ Value::ATTR_FLAG == key_tag & Value::TYPE_MASK
    }

    /// Returns `true` if this `Item` is a `Value`.
    pub fn is_value(&self) -> bool {
        self.key_tag() == 0
    }

    /// Returns `true` if this `Item` is a `Record`.
    pub fn is_record(&self) -> bool {
        self.is_value() && self.val.is_record()
    }

    /// Returns `true` if this `Item` is `Data`.
    pub fn is_data(&self) -> bool {
        self.is_value() && self.val.is_data()
    }

    /// Returns `true` if this `Item` is `Text`.
    pub fn is_text(&self) -> bool {
        self.is_value() && self.val.is_text()
    }

    /// Returns `true` if this `Item` is a `Num`.
    pub fn is_num(&self) -> bool {
        self.is_value() && self.val.is_num()
    }

    /// Returns `true` if this `Item` is a `Bool`.
    pub fn is_bool(&self) -> bool {
        self.is_value() && self.val.is_bool()
    }

    /// Returns `true` if this `Item` is `Extant`.
    pub fn is_extant(&self) -> bool {
        self.is_value() && self.val.is_extant()
    }

    /// Returns `true` if this `Item` is `Absent`.
    pub fn is_absent(&self) -> bool {
        self.is_value() && self.val.is_absent()
    }

    /// Downcasts this `Item` reference to a `Field` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Field`.
    #[inline]
    pub fn as_field(&self) -> &Field<'a> {
        if self.is_field() {
            unsafe { self.as_field_unchecked() }
        } else {
            panic!("not a Field");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Field` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Field`.
    #[inline]
    pub fn as_mut_field(&mut self) -> &mut Field<'a> {
        if self.is_field() {
            unsafe { self.as_mut_field_unchecked() }
        } else {
            panic!("not a Field");
        }
    }

    /// Downcasts this `Item` reference to an `Attr` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, an `Attr`.
    #[inline]
    pub fn as_attr(&self) -> &Attr<'a> {
        if self.is_attr() {
            unsafe { self.as_attr_unchecked() }
        } else {
            panic!("not an Attr");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Attr` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, an `Attr`.
    #[inline]
    pub fn as_mut_attr(&mut self) -> &mut Attr<'a> {
        if self.is_attr() {
            unsafe { self.as_mut_attr_unchecked() }
        } else {
            panic!("not an Attr");
        }
    }

    /// Downcasts this `Item` reference to a `Slot` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Slot`.
    #[inline]
    pub fn as_slot(&self) -> &Slot<'a> {
        if self.is_slot() {
            unsafe { self.as_slot_unchecked() }
        } else {
            panic!("not a Slot");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Slot` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Slot`.
    #[inline]
    pub fn as_mut_slot(&mut self) -> &mut Slot<'a> {
        if self.is_slot() {
            unsafe { self.as_mut_slot_unchecked() }
        } else {
            panic!("not a Slot");
        }
    }

    /// Downcasts this `Item` reference to a `Value` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Value`.
    #[inline]
    pub fn as_value(&self) -> &Value<'a> {
        if self.is_value() {
            &self.val
        } else {
            panic!("not a Value");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Value` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Value`.
    #[inline]
    pub fn as_mut_value(&mut self) -> &mut Value<'a> {
        if self.is_value() {
            &mut self.val
        } else {
            panic!("not a Value");
        }
    }

    /// Downcasts this `Item` reference to a `Record` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Record`.
    #[inline]
    pub fn as_record(&self) -> &Record<'a> {
        if self.is_record() {
            unsafe { self.as_record_unchecked() }
        } else {
            panic!("not a Record");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Record` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Record`.
    #[inline]
    pub fn as_mut_record(&mut self) -> &mut Record<'a> {
        if self.is_record() {
            unsafe { self.as_mut_record_unchecked() }
        } else {
            panic!("not a Record");
        }
    }

    /// Downcasts this `Item` reference to a `Data` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Data`.
    #[inline]
    pub fn as_data(&self) -> &Data<'a> {
        if self.is_data() {
            unsafe { self.as_data_unchecked() }
        } else {
            panic!("not Data");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Data` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Data`.
    #[inline]
    pub fn as_mut_data(&mut self) -> &mut Data<'a> {
        if self.is_data() {
            unsafe { self.as_mut_data_unchecked() }
        } else {
            panic!("not Data");
        }
    }

    /// Downcasts this `Item` reference to a `Text` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Text`.
    #[inline]
    pub fn as_text(&self) -> &Text<'a> {
        if self.is_text() {
            unsafe { self.as_text_unchecked() }
        } else {
            panic!("not Text");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Text` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Text`.
    #[inline]
    pub fn as_mut_text(&mut self) -> &mut Text<'a> {
        if self.is_text() {
            unsafe { self.as_mut_text_unchecked() }
        } else {
            panic!("not Text");
        }
    }

    /// Downcasts this `Item` reference to a `Num` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Num`.
    #[inline]
    pub fn as_num(&self) -> &Num<'a> {
        if self.is_num() {
            unsafe { self.as_num_unchecked() }
        } else {
            panic!("not a Num");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Num` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Num`.
    #[inline]
    pub fn as_mut_num(&mut self) -> &mut Num<'a> {
        if self.is_num() {
            unsafe { self.as_mut_num_unchecked() }
        } else {
            panic!("not a Num");
        }
    }

    /// Downcasts this `Item` reference to a `Bool` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Bool`.
    #[inline]
    pub fn as_bool(&self) -> &Bool {
        if self.is_bool() {
            unsafe { self.as_bool_unchecked() }
        } else {
            panic!("not a Bool");
        }
    }

    /// Downcasts this `Item` reference to a mutable `Bool` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Bool`.
    #[inline]
    pub fn as_mut_bool(&mut self) -> &mut Bool {
        if self.is_bool() {
            unsafe { self.as_mut_bool_unchecked() }
        } else {
            panic!("not a Bool");
        }
    }

    /// Downcasts this `Item` reference to an `Extant` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Extant`.
    #[inline]
    pub fn as_extant(&self) -> &Extant {
        if self.is_extant() {
            unsafe { self.as_extant_unchecked() }
        } else {
            panic!("not Extant");
        }
    }

    /// Downcasts this `Item` reference to an `Absent` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Absent`.
    #[inline]
    pub fn as_absent(&self) -> &Absent {
        if self.is_absent() {
            unsafe { self.as_absent_unchecked() }
        } else {
            panic!("not Absent");
        }
    }

    /// Reinterprets this `Item` reference as an `Field` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, an `Field`.
    #[inline]
    pub unsafe fn as_field_unchecked(&self) -> &Field<'a> {
        mem::transmute::<&Item<'a>, &Field<'a>>(self)
    }

    /// Reinterprets this `Item` reference as a mutable `Field` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, an `Field`.
    #[inline]
    pub unsafe fn as_mut_field_unchecked(&mut self) -> &mut Field<'a> {
        mem::transmute::<&mut Item<'a>, &mut Field<'a>>(self)
    }

    /// Reinterprets this `Item` reference as an `Attr` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, an `Attr`.
    #[inline]
    pub unsafe fn as_attr_unchecked(&self) -> &Attr<'a> {
        mem::transmute::<&Item<'a>, &Attr<'a>>(self)
    }

    /// Reinterprets this `Item` reference as a mutable `Attr` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, an `Attr`.
    #[inline]
    pub unsafe fn as_mut_attr_unchecked(&mut self) -> &mut Attr<'a> {
        mem::transmute::<&mut Item<'a>, &mut Attr<'a>>(self)
    }

    /// Reinterprets this `Item` reference as a `Slot` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Slot`.
    #[inline]
    pub unsafe fn as_slot_unchecked(&self) -> &Slot<'a> {
        mem::transmute::<&Item<'a>, &Slot<'a>>(self)
    }

    /// Reinterprets this `Item` reference as a mutable `Slot` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Slot`.
    #[inline]
    pub unsafe fn as_mut_slot_unchecked(&mut self) -> &mut Slot<'a> {
        mem::transmute::<&mut Item<'a>, &mut Slot<'a>>(self)
    }

    /// Reinterprets this `Item` reference as a `Record` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Record`.
    #[inline]
    pub unsafe fn as_record_unchecked(&self) -> &Record<'a> {
        self.val.as_record_unchecked()
    }

    /// Reinterprets this `Item` reference as a mutable `Record` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Record`.
    #[inline]
    pub unsafe fn as_mut_record_unchecked(&mut self) -> &mut Record<'a> {
        self.val.as_mut_record_unchecked()
    }

    /// Reinterprets this `Item` reference as a `Data` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, `Data`.
    #[inline]
    pub unsafe fn as_data_unchecked(&self) -> &Data<'a> {
        self.val.as_data_unchecked()
    }

    /// Reinterprets this `Item` reference as a mutable `Data` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, `Data`.
    #[inline]
    pub unsafe fn as_mut_data_unchecked(&mut self) -> &mut Data<'a> {
        self.val.as_mut_data_unchecked()
    }

    /// Reinterprets this `Item` reference as a `Text` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, `Text`.
    #[inline]
    pub unsafe fn as_text_unchecked(&self) -> &Text<'a> {
        self.val.as_text_unchecked()
    }

    /// Reinterprets this `Item` reference as a `Text` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, `Text`.
    #[inline]
    pub unsafe fn as_mut_text_unchecked(&mut self) -> &mut Text<'a> {
        self.val.as_mut_text_unchecked()
    }

    /// Reinterprets this `Item` reference as a `Num` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Num`.
    #[inline]
    pub unsafe fn as_num_unchecked(&self) -> &Num<'a> {
        self.val.as_num_unchecked()
    }

    /// Reinterprets this `Item` reference as a `Num` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Num`.
    #[inline]
    pub unsafe fn as_mut_num_unchecked(&mut self) -> &mut Num<'a> {
        self.val.as_mut_num_unchecked()
    }

    /// Reinterprets this `Item` reference as a `Bool` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Bool`.
    #[inline]
    pub unsafe fn as_bool_unchecked(&self) -> &Bool {
        self.val.as_bool_unchecked()
    }

    /// Reinterprets this `Item` reference as a `Bool` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Bool`.
    #[inline]
    pub unsafe fn as_mut_bool_unchecked(&mut self) -> &mut Bool {
        self.val.as_mut_bool_unchecked()
    }

    /// Reinterprets this `Item` reference as an `Extant` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, `Extant`.
    #[inline]
    pub unsafe fn as_extant_unchecked(&self) -> &Extant {
        self.val.as_extant_unchecked()
    }

    /// Reinterprets this `Item` reference as an `Absent` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, `Absent`.
    #[inline]
    pub unsafe fn as_absent_unchecked(&self) -> &Absent {
        self.val.as_absent_unchecked()
    }

    /// Downcasts this `Item` to a `Field`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Field`.
    #[inline]
    pub fn into_field(self) -> Field<'a> {
        if self.is_field() {
            unsafe { self.into_field_unchecked() }
        } else {
            panic!("not a Field");
        }
    }

    /// Downcasts this `Item` to an `Attr`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, an `Attr`.
    #[inline]
    pub fn into_attr(self) -> Attr<'a> {
        if self.is_attr() {
            unsafe { self.into_attr_unchecked() }
        } else {
            panic!("not an Attr");
        }
    }

    /// Downcasts this `Item` to a `Slot`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Slot`.
    #[inline]
    pub fn into_slot(self) -> Slot<'a> {
        if self.is_slot() {
            unsafe { self.into_slot_unchecked() }
        } else {
            panic!("not a Slot");
        }
    }

    /// Downcasts this `Item` to a `Value`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Value`.
    #[inline]
    pub fn into_value(self) -> Value<'a> {
        if self.is_value() {
            self.into_val()
        } else {
            panic!("not a Value");
        }
    }

    /// Downcasts this `Item` to a `Record`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Record`.
    #[inline]
    pub fn into_record(self) -> Record<'a> {
        if self.is_record() {
            unsafe { self.into_record_unchecked() }
        } else {
            panic!("not a Record");
        }
    }

    /// Downcasts this `Item` to `Data`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Data`.
    #[inline]
    pub fn into_data(self) -> Data<'a> {
        if self.is_data() {
            unsafe { self.into_data_unchecked() }
        } else {
            panic!("not Data");
        }
    }

    /// Downcasts this `Item` to `Text`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Text`.
    #[inline]
    pub fn into_text(self) -> Text<'a> {
        if self.is_text() {
            unsafe { self.into_text_unchecked() }
        } else {
            panic!("not Text");
        }
    }

    /// Downcasts this `Item` to a `Num`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Num`.
    #[inline]
    pub fn into_num(self) -> Num<'a> {
        if self.is_num() {
            unsafe { self.into_num_unchecked() }
        } else {
            panic!("not a Num");
        }
    }

    /// Downcasts this `Item` to a `Bool`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, a `Bool`.
    #[inline]
    pub fn into_bool(self) -> Bool {
        if self.is_bool() {
            unsafe { self.into_bool_unchecked() }
        } else {
            panic!("not a Bool");
        }
    }

    /// Downcasts this `Item` to `Extant`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Extant`.
    #[inline]
    pub fn into_extant(self) -> Extant {
        if self.is_extant() {
            unsafe { self.into_extant_unchecked() }
        } else {
            panic!("not Extant");
        }
    }

    /// Downcasts this `Item` to `Absent`.
    ///
    /// # Panics
    ///
    /// Panics if this `Item` is not, in fact, `Absent`.
    #[inline]
    pub fn into_absent(self) -> Absent {
        if self.is_absent() {
            unsafe { self.into_absent_unchecked() }
        } else {
            panic!("not Absent");
        }
    }

    /// Reinterprets this `Item` as a `Field`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Field`.
    #[inline]
    pub unsafe fn into_field_unchecked(self) -> Field<'a> {
        mem::transmute::<Item<'a>, Field<'a>>(self)
    }

    /// Reinterprets this `Item` as an `Attr`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, an `Attr`.
    #[inline]
    pub unsafe fn into_attr_unchecked(self) -> Attr<'a> {
        mem::transmute::<Item<'a>, Attr<'a>>(self)
    }

    /// Reinterprets this `Item` as a `Slot`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Item` is, in fact, a `Slot`.
    #[inline]
    pub unsafe fn into_slot_unchecked(self) -> Slot<'a> {
        mem::transmute::<Item<'a>, Slot<'a>>(self)
    }

    /// Reinterprets this `Value` as a `Record`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Record`.
    #[inline]
    pub unsafe fn into_record_unchecked(self) -> Record<'a> {
        self.val.into_record_unchecked()
    }

    /// Reinterprets this `Value` as `Data`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Data`.
    #[inline]
    pub unsafe fn into_data_unchecked(self) -> Data<'a> {
       self.val.into_data_unchecked()
    }

    /// Reinterprets this `Value` as `Text`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Text`.
    #[inline]
    pub unsafe fn into_text_unchecked(self) -> Text<'a> {
        self.val.into_text_unchecked()
    }

    /// Reinterprets this `Value` as a `Num`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Num`.
    #[inline]
    pub unsafe fn into_num_unchecked(self) -> Num<'a> {
        self.val.into_num_unchecked()
    }

    /// Reinterprets this `Value` as a `Bool`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, a `Bool`.
    #[inline]
    pub unsafe fn into_bool_unchecked(self) -> Bool {
        self.val.into_bool_unchecked()
    }

    /// Reinterprets this `Value` as `Extant`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Extant`.
    #[inline]
    pub unsafe fn into_extant_unchecked(self) -> Extant {
        self.val.into_extant_unchecked()
    }

    /// Reinterprets this `Value` as `Absent`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Value` is, in fact, `Absent`.
    #[inline]
    pub unsafe fn into_absent_unchecked(self) -> Absent {
        self.val.into_absent_unchecked()
    }

    /// Downcasts this `Item` reference to a typechecked `Field` reference.
    #[inline]
    pub fn cast_as_field(&self) -> Option<&Field<'a>> {
        if self.is_field() {
            Some(unsafe { self.as_field_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Field` reference.
    #[inline]
    pub fn cast_as_mut_field(&mut self) -> Option<&mut Field<'a>> {
        if self.is_field() {
            Some(unsafe { self.as_mut_field_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Attr` reference.
    #[inline]
    pub fn cast_as_attr(&self) -> Option<&Attr<'a>> {
        if self.is_attr() {
            Some(unsafe { self.as_attr_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Attr` reference.
    #[inline]
    pub fn cast_as_mut_attr(&mut self) -> Option<&mut Attr<'a>> {
        if self.is_attr() {
            Some(unsafe { self.as_mut_attr_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Slot` reference.
    #[inline]
    pub fn cast_as_slot(&self) -> Option<&Slot<'a>> {
        if self.is_slot() {
            Some(unsafe { self.as_slot_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Slot` reference.
    #[inline]
    pub fn cast_as_mut_slot(&mut self) -> Option<&mut Slot<'a>> {
        if self.is_slot() {
            Some(unsafe { self.as_mut_slot_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Value` reference.
    #[inline]
    pub fn cast_as_value(&self) -> Option<&Value<'a>> {
        if self.is_value() {
            Some(&self.val)
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Value` reference.
    #[inline]
    pub fn cast_as_mut_value(&mut self) -> Option<&mut Value<'a>> {
        if self.is_value() {
            Some(&mut self.val)
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Record` reference.
    #[inline]
    pub fn cast_as_record(&self) -> Option<&Record<'a>> {
        if self.is_record() {
            Some(unsafe { self.as_record_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Record` reference.
    #[inline]
    pub fn cast_as_mut_record(&mut self) -> Option<&mut Record<'a>> {
        if self.is_record() {
            Some(unsafe { self.as_mut_record_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Data` reference.
    #[inline]
    pub fn cast_as_data(&self) -> Option<&Data<'a>> {
        if self.is_data() {
            Some(unsafe { self.as_data_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Data` reference.
    #[inline]
    pub fn cast_as_mut_data(&mut self) -> Option<&mut Data<'a>> {
        if self.is_data() {
            Some(unsafe { self.as_mut_data_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Text` reference.
    #[inline]
    pub fn cast_as_text(&self) -> Option<&Text<'a>> {
        if self.is_text() {
            Some(unsafe { self.as_text_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Text` reference.
    #[inline]
    pub fn cast_as_mut_text(&mut self) -> Option<&mut Text<'a>> {
        if self.is_text() {
            Some(unsafe { self.as_mut_text_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Num` reference.
    #[inline]
    pub fn cast_as_num(&self) -> Option<&Num<'a>> {
        if self.is_num() {
            Some(unsafe { self.as_num_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Num` reference.
    #[inline]
    pub fn cast_as_mut_num(&mut self) -> Option<&mut Num<'a>> {
        if self.is_num() {
            Some(unsafe { self.as_mut_num_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Bool` reference.
    #[inline]
    pub fn cast_as_bool(&self) -> Option<&Bool> {
        if self.is_bool() {
            Some(unsafe { self.as_bool_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked, mutable `Bool` reference.
    #[inline]
    pub fn cast_as_mut_bool(&mut self) -> Option<&mut Bool> {
        if self.is_bool() {
            Some(unsafe { self.as_mut_bool_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Extant` reference.
    #[inline]
    pub fn cast_as_extant(&self) -> Option<&Extant> {
        if self.is_extant() {
            Some(unsafe { self.as_extant_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` reference to a typechecked `Absent` reference.
    #[inline]
    pub fn cast_as_absent(&self) -> Option<&Absent> {
        if self.is_absent() {
            Some(unsafe { self.as_absent_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to a typechecked `Field`.
    #[inline]
    pub fn cast_into_field(self) -> Option<Field<'a>> {
        if self.is_field() {
            Some(unsafe { self.into_field_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to a typechecked `Attr`.
    #[inline]
    pub fn cast_into_attr(self) -> Option<Attr<'a>> {
        if self.is_attr() {
            Some(unsafe { self.into_attr_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to a typechecked `Slot`.
    #[inline]
    pub fn cast_into_slot(self) -> Option<Slot<'a>> {
        if self.is_slot() {
            Some(unsafe { self.into_slot_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to a typechecked `Value`.
    #[inline]
    pub fn cast_into_value(self) -> Option<Value<'a>> {
        if self.is_value() {
            Some(self.into_val())
        } else {
            None
        }
    }

    /// Downcasts this `Item` to a typechecked `Record`.
    #[inline]
    pub fn cast_into_record(self) -> Option<Record<'a>> {
        if self.is_record() {
            Some(unsafe { self.into_record_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to typechecked `Data`.
    #[inline]
    pub fn cast_into_data(self) -> Option<Data<'a>> {
        if self.is_data() {
            Some(unsafe { self.into_data_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to typechecked `Text`.
    #[inline]
    pub fn cast_into_text(self) -> Option<Text<'a>> {
        if self.is_text() {
            Some(unsafe { self.into_text_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to a typechecked `Num`.
    #[inline]
    pub fn cast_into_num(self) -> Option<Num<'a>> {
        if self.is_num() {
            Some(unsafe { self.into_num_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to a typechecked `Bool`.
    #[inline]
    pub fn cast_into_bool(self) -> Option<Bool> {
        if self.is_bool() {
            Some(unsafe { self.into_bool_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to typechecked `Extant`.
    #[inline]
    pub fn cast_into_extant(self) -> Option<Extant> {
        if self.is_extant() {
            Some(unsafe { self.into_extant_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Item` to typechecked `Absent`.
    #[inline]
    pub fn cast_into_absent(self) -> Option<Absent> {
        if self.is_absent() {
            Some(unsafe { self.into_absent_unchecked() })
        } else {
            None
        }
    }
}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, that: &Item<'a>) -> bool {
        self.key.eq(&that.key) && self.val.eq(&that.val)
    }

    fn ne(&self, that: &Item<'a>) -> bool {
        self.key.ne(&that.key) || self.val.ne(&that.val)
    }
}

impl<'a> cmp::PartialOrd<Item<'a>> for Item<'a> {
    fn partial_cmp(&self, _that: &Item<'a>) -> Option<cmp::Ordering> {
        unimplemented!(); // TODO
    }

    fn lt(&self, _that: &Item<'a>) -> bool {
        unimplemented!(); // TODO
    }

    fn le(&self, _that: &Item<'a>) -> bool {
        unimplemented!(); // TODO
    }

    fn ge(&self, _that: &Item<'a>) -> bool {
        unimplemented!(); // TODO
    }

    fn gt(&self, _that: &Item<'a>) -> bool {
        unimplemented!(); // TODO
    }
}

impl<'a> hash::Hash for Item<'a> {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.key.hash(hasher);
        self.val.hash(hasher);
    }
}

impl<'a> fmt::Debug for Item<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!(); // TODO
    }
}

impl<'a> Clone for Item<'a> {
    fn clone(&self) -> Item<'a> {
        let item = Item {
            val: self.val.clone(),
            key: self.key.clone(),
        };
        if self.is_attr() {
            unsafe { ptr::write(item.key_tag_ptr(), *self.key_tag_ptr() | Value::ATTR_FLAG); }
        }
        item
    }
}

impl<'a> TryClone for Item<'a> {
    fn try_clone(&self) -> Result<Item<'a>, HoldError> {
        let item = Item {
            val: self.val.try_clone()?,
            key: self.key.try_clone()?,
        };
        if self.is_attr() {
            unsafe { ptr::write(item.key_tag_ptr(), *self.key_tag_ptr() | Value::ATTR_FLAG); }
        }
        Ok(item)
    }
}

impl<'a, 'b> CloneIntoHold<'a, Item<'a>> for Item<'b> {
    fn try_clone_into_hold(&self, hold: &Hold<'a>) -> Result<Item<'a>, HoldError> {
        let item = Item {
            val: self.val.try_clone_into_hold(hold)?,
            key: self.key.try_clone_into_hold(hold)?,
        };
        if self.is_attr() {
            unsafe { ptr::write(item.key_tag_ptr(), *self.key_tag_ptr() | Value::ATTR_FLAG); }
        }
        Ok(item)
    }
}

impl<'a, 'b> Stow<'b, Item<'b>> for Item<'a> {
    unsafe fn stow(src: *mut Item<'a>, dst: *mut Item<'b>, hold: &Hold<'b>) -> Result<(), HoldError> {
        if let err @ Err(_) = Value::stow(&mut (*src).val, &mut (*dst).val, hold) {
            return err;
        }
        if let Some(key) = &mut (*src).key {
            if let err @ Err(_) = Value::stow(&mut *key, &mut (*dst).key as *mut Option<Value<'b>> as *mut Value<'b>, hold) {
                Value::unstow(&mut (*src).val, &mut (*dst).val);
                return err;
            }
        }
        Ok(())
    }

    unsafe fn unstow(src: *mut Item<'a>, dst: *mut Item<'b>) {
        if let Some(key) = &mut (*dst).key {
            Value::unstow(&mut (*src).key as *mut Option<Value<'a>> as *mut Value<'a>, &mut *key);
        }
        Value::unstow(&mut (*src).val, &mut (*dst).val);
    }
}

impl<'a> Default for Item<'a> {
    #[inline]
    fn default() -> Item<'a> {
        Item::absent()
    }
}

impl<'a> From<Field<'a>> for Item<'a> {
    #[inline]
    fn from(field: Field<'a>) -> Item<'a> {
        field.into_item()
    }
}

impl<'a> From<Attr<'a>> for Item<'a> {
    #[inline]
    fn from(attr: Attr<'a>) -> Item<'a> {
        attr.into_item()
    }
}

impl<'a> From<Slot<'a>> for Item<'a> {
    #[inline]
    fn from(slot: Slot<'a>) -> Item<'a> {
        slot.into_item()
    }
}

impl<'a> From<Value<'a>> for Item<'a> {
    #[inline]
    fn from(value: Value<'a>) -> Item<'a> {
        value.into_item()
    }
}

impl<'a> From<Record<'a>> for Item<'a> {
    #[inline]
    fn from(record: Record<'a>) -> Item<'a> {
        record.into_item()
    }
}

impl<'a> From<Data<'a>> for Item<'a> {
    #[inline]
    fn from(data: Data<'a>) -> Item<'a> {
        data.into_item()
    }
}

impl<'a> From<Text<'a>> for Item<'a> {
    #[inline]
    fn from(text: Text<'a>) -> Item<'a> {
        text.into_item()
    }
}

impl<'a> From<Num<'a>> for Item<'a> {
    #[inline]
    fn from(num: Num<'a>) -> Item<'a> {
        num.into_item()
    }
}

impl<'a> From<Bool> for Item<'a> {
    #[inline]
    fn from(bool: Bool) -> Item<'a> {
        bool.into_item()
    }
}

impl<'a> From<Extant> for Item<'a> {
    #[inline]
    fn from(extant: Extant) -> Item<'a> {
        extant.into_item()
    }
}

impl<'a> From<Absent> for Item<'a> {
    #[inline]
    fn from(absent: Absent) -> Item<'a> {
        absent.into_item()
    }
}

impl<'a> From<u8> for Item<'a> {
    fn from(value: u8) -> Item<'a> {
        Item::from_u8(value)
    }
}

impl<'a> From<i8> for Item<'a> {
    fn from(value: i8) -> Item<'a> {
        Item::from_i8(value)
    }
}

impl<'a> From<u16> for Item<'a> {
    fn from(value: u16) -> Item<'a> {
        Item::from_u16(value)
    }
}

impl<'a> From<i16> for Item<'a> {
    fn from(value: i16) -> Item<'a> {
        Item::from_i16(value)
    }
}

impl<'a> From<u32> for Item<'a> {
    fn from(value: u32) -> Item<'a> {
        Item::from_u32(value)
    }
}

impl<'a> From<i32> for Item<'a> {
    fn from(value: i32) -> Item<'a> {
        Item::from_i32(value)
    }
}

impl<'a> From<u64> for Item<'a> {
    fn from(value: u64) -> Item<'a> {
        Item::from_u64(value)
    }
}

impl<'a> From<i64> for Item<'a> {
    fn from(value: i64) -> Item<'a> {
        Item::from_i64(value)
    }
}

impl<'a> From<f16> for Item<'a> {
    fn from(value: f16) -> Item<'a> {
        Item::from_f16(value)
    }
}

impl<'a> From<f32> for Item<'a> {
    fn from(value: f32) -> Item<'a> {
        Item::from_f32(value)
    }
}

impl<'a> From<f64> for Item<'a> {
    fn from(value: f64) -> Item<'a> {
        Item::from_f64(value)
    }
}

impl<'a> From<usize> for Item<'a> {
    fn from(value: usize) -> Item<'a> {
        Item::from_usize(value)
    }
}

impl<'a> From<isize> for Item<'a> {
    fn from(value: isize) -> Item<'a> {
        Item::from_isize(value)
    }
}

impl<'a> From<bool> for Item<'a> {
    #[inline]
    fn from(value: bool) -> Item<'a> {
        Item::from_bool(value)
    }
}
