use core::fmt;
use core::hash;
use core::mem;
use core::ptr;
use crate::item::{Item, Attr, Slot, Value};

/// `Item` variant representing a key-value pair with a `Value`-typed key
/// and value.
#[derive(Eq)]
#[repr(C)]
pub struct Field<'a> {
    val: Value<'a>,
    key: Value<'a>,
}

impl<'a> Field<'a> {
    /// Returns a reference to this `Field`'s key.
    #[inline]
    pub fn get_key(&self) -> &Value<'a> {
        &self.key
    }

    /// Returns a reference to this `Field`'s value.
    #[inline]
    pub fn get_val(&self) -> &Value<'a> {
        &self.val
    }

    /// Returns a mutable reference to this `Field`'s value.
    #[inline]
    pub fn get_val_mut(&mut self) -> &mut Value<'a> {
        &mut self.val
    }

    /// Returns a pair of references to this `Field`'s key and value.
    #[inline]
    pub fn get_key_val(&self) -> (&Value<'a>, &Value<'a>) {
        (&self.key, &self.val)
    }

    /// Returns a pair of references to this `Field`'s key and mutable value.
    #[inline]
    pub fn get_key_val_mut(&mut self) -> (&Value<'a>, &mut Value<'a>) {
        (&self.key, &mut self.val)
    }

    /// Returns this `Field`'s key, dropping its value.
    #[inline]
    pub fn into_key(mut self) -> Value<'a> {
        let key = unsafe { ptr::read(&self.key) };
        unsafe { ptr::drop_in_place(&mut self.val); }
        mem::forget(self);
        key
    }

    /// Returns this `Field`'s value, dropping its key.
    #[inline]
    pub fn into_val(mut self) -> Value<'a> {
        unsafe { ptr::drop_in_place(&mut self.key) };
        let val = unsafe { ptr::read(&self.val) };
        mem::forget(self);
        val
    }

    /// Returns this `Field`'s key and value as a pair.
    #[inline]
    pub fn into_key_val(self) -> (Value<'a>, Value<'a>) {
        let key = unsafe { ptr::read(&self.key) };
        let val = unsafe { ptr::read(&self.val) };
        mem::forget(self);
        (key, val)
    }

    /// Returns `true` if this `Field` is an `Attr`.
    pub fn is_attr(&self) -> bool {
        self.key.tag() & Value::ATTR_FLAG != 0
    }

    /// Returns `true` if this `Field` is a `Slot`.
    pub fn is_slot(&self) -> bool {
        self.key.tag() & Value::ATTR_FLAG == 0
    }

    /// Downcasts this `Field` reference to an `Attr` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Field` is not, in fact, an `Attr`.
    #[inline]
    pub fn as_attr(&self) -> &Attr<'a> {
        if self.is_attr() {
            unsafe { self.as_attr_unchecked() }
        } else {
            panic!("not an Attr");
        }
    }

    /// Downcasts this `Field` reference to a mutable `Attr` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Field` is not, in fact, an `Attr`.
    #[inline]
    pub fn as_mut_attr(&mut self) -> &mut Attr<'a> {
        if self.is_attr() {
            unsafe { self.as_mut_attr_unchecked() }
        } else {
            panic!("not an Attr");
        }
    }

    /// Downcasts this `Field` reference to a `Slot` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Field` is not, in fact, a `Slot`.
    #[inline]
    pub fn as_slot(&self) -> &Slot<'a> {
        if self.is_slot() {
            unsafe { self.as_slot_unchecked() }
        } else {
            panic!("not a Slot");
        }
    }

    /// Downcasts this `Field` reference to a mutable `Slot` reference.
    ///
    /// # Panics
    ///
    /// Panics if this `Field` is not, in fact, a `Slot`.
    #[inline]
    pub fn as_mut_slot(&mut self) -> &mut Slot<'a> {
        if self.is_slot() {
            unsafe { self.as_mut_slot_unchecked() }
        } else {
            panic!("not a Slot");
        }
    }

    /// Reinterprets this `Field` reference as an `Attr` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Field` is, in fact, an `Attr`.
    #[inline]
    pub unsafe fn as_attr_unchecked(&self) -> &Attr<'a> {
        mem::transmute::<&Field<'a>, &Attr<'a>>(self)
    }

    /// Reinterprets this `Field` reference as a mutable `Attr` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Field` is, in fact, an `Attr`.
    #[inline]
    pub unsafe fn as_mut_attr_unchecked(&mut self) -> &mut Attr<'a> {
        mem::transmute::<&mut Field<'a>, &mut Attr<'a>>(self)
    }

    /// Reinterprets this `Field` reference as a `Slot` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Field` is, in fact, a `Slot`.
    #[inline]
    pub unsafe fn as_slot_unchecked(&self) -> &Slot<'a> {
        mem::transmute::<&Field<'a>, &Slot<'a>>(self)
    }

    /// Reinterprets this `Field` reference as a mutable `Slot` reference.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Field` is, in fact, a `Slot`.
    #[inline]
    pub unsafe fn as_mut_slot_unchecked(&mut self) -> &mut Slot<'a> {
        mem::transmute::<&mut Field<'a>, &mut Slot<'a>>(self)
    }

    /// Upcasts this `Field` reference to an `Item` reference.
    #[inline]
    pub fn as_item(&self) -> &Item<'a> {
        unsafe { mem::transmute::<&Field<'a>, &Item<'a>>(self) }
    }

    /// Upcasts this `Field` reference to a mutable `Item` reference.
    #[inline]
    pub fn as_mut_item(&mut self) -> &mut Item<'a> {
        unsafe { mem::transmute::<&mut Field<'a>, &mut Item<'a>>(self) }
    }

    /// Downcasts this `Field` to an `Attr`.
    ///
    /// # Panics
    ///
    /// Panics if this `Field` is not, in fact, an `Attr`.
    #[inline]
    pub fn into_attr(self) -> Attr<'a> {
        if self.is_attr() {
            unsafe { self.into_attr_unchecked() }
        } else {
            panic!("not an Attr");
        }
    }

    /// Downcasts this `Field` to a `Slot`.
    ///
    /// # Panics
    ///
    /// Panics if this `Field` is not, in fact, a `Slot`.
    #[inline]
    pub fn into_slot(self) -> Slot<'a> {
        if self.is_slot() {
            unsafe { self.into_slot_unchecked() }
        } else {
            panic!("not a Slot");
        }
    }

    /// Reinterprets this `Field` as an `Attr`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Field` is, in fact, an `Attr`.
    #[inline]
    pub unsafe fn into_attr_unchecked(self) -> Attr<'a> {
        mem::transmute::<Field<'a>, Attr<'a>>(self)
    }

    /// Reinterprets this `Field` as a `Slot`.
    ///
    /// # Safety
    ///
    /// Does not verify that this `Field` is, in fact, a `Slot`.
    #[inline]
    pub unsafe fn into_slot_unchecked(self) -> Slot<'a> {
        mem::transmute::<Field<'a>, Slot<'a>>(self)
    }

    /// Upcasts this `Field` to an `Item`.
    #[inline]
    pub fn into_item(self) -> Item<'a> {
        unsafe { mem::transmute::<Field<'a>, Item<'a>>(self) }
    }

    /// Downcasts this `Field` reference to a typechecked `Attr` reference.
    #[inline]
    pub fn cast_as_attr(&self) -> Option<&Attr<'a>> {
        if self.is_attr() {
            Some(unsafe { self.as_attr_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Field` reference to a typechecked, mutable `Attr` reference.
    #[inline]
    pub fn cast_as_mut_attr(&mut self) -> Option<&mut Attr<'a>> {
        if self.is_attr() {
            Some(unsafe { self.as_mut_attr_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Field` reference to a typechecked `Slot` reference.
    #[inline]
    pub fn cast_as_slot(&self) -> Option<&Slot<'a>> {
        if self.is_slot() {
            Some(unsafe { self.as_slot_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Field` reference to a typechecked, mutable `Slot` reference.
    #[inline]
    pub fn cast_as_mut_slot(&mut self) -> Option<&mut Slot<'a>> {
        if self.is_slot() {
            Some(unsafe { self.as_mut_slot_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Field` to a typechecked `Attr`.
    #[inline]
    pub fn cast_into_attr(self) -> Option<Attr<'a>> {
        if self.is_attr() {
            Some(unsafe { self.into_attr_unchecked() })
        } else {
            None
        }
    }

    /// Downcasts this `Field` to a typechecked `Slot`.
    #[inline]
    pub fn cast_into_slot(self) -> Option<Slot<'a>> {
        if self.is_slot() {
            Some(unsafe { self.into_slot_unchecked() })
        } else {
            None
        }
    }
}

impl<'a> AsRef<Item<'a>> for Field<'a> {
    #[inline]
    fn as_ref(&self) -> &Item<'a> {
        self.as_item()
    }
}

impl<'a> AsMut<Item<'a>> for Field<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Item<'a> {
        self.as_mut_item()
    }
}

impl<'a> PartialEq for Field<'a> {
    fn eq(&self, _that: &Field<'a>) -> bool {
        unimplemented!(); // TODO
    }
}

impl<'a> hash::Hash for Field<'a> {
    fn hash<H: hash::Hasher>(&self, _hasher: &mut H) {
        unimplemented!(); // TODO
    }
}

impl<'a> fmt::Debug for Field<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!(); // TODO
    }
}

impl<'a> From<Attr<'a>> for Field<'a> {
    #[inline]
    fn from(attr: Attr<'a>) -> Field<'a> {
        attr.into_field()
    }
}

impl<'a> From<Slot<'a>> for Field<'a> {
    #[inline]
    fn from(slot: Slot<'a>) -> Field<'a> {
        slot.into_field()
    }
}
