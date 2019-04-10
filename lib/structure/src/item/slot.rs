use core::fmt::{self, Write};
use core::hash;
use core::mem;
use core::ptr;
use crate::item::{Item, Field, Value};

/// `Field` variant representing a key-value member with a `Value`-typed key
/// and value.
///
/// # Examples
///
/// Construct a new `Slot` field:
///
/// ```
/// # extern crate swim_c_rt;
/// # use swim_structure::item::{Slot, Value};
/// let field = Slot::new(Value::from_str("a"), Value::from(true));
/// # assert!(!field.as_item().is_attr());
/// # assert_eq!(field.get_key().as_text().as_str(), "a");
/// # assert_eq!(field.get_val().as_bool().to_bool(), true);
/// ```
#[derive(Eq)]
#[repr(C)]
pub struct Slot<'a> {
    val: Value<'a>,
    key: Value<'a>,
}

impl<'a> Slot<'a> {
    pub fn new(key: Value<'a>, val: Value<'a>) -> Slot<'a> {
        let slot = Slot {
            val: val,
            key: key,
        };
        unsafe {
            let key_tag_ptr = slot.key.tag_ptr();
            ptr::write(key_tag_ptr, *key_tag_ptr & !Value::ATTR_FLAG);
        }
        slot
    }

    /// Returns a reference to this `Slot`'s key.
    #[inline]
    pub fn get_key(&self) -> &Value<'a> {
        &self.key
    }

    /// Returns a reference to this `Slot`'s value.
    #[inline]
    pub fn get_val(&self) -> &Value<'a> {
        &self.val
    }

    /// Returns a mutable reference to this `Slot`'s value.
    #[inline]
    pub fn get_val_mut(&mut self) -> &mut Value<'a> {
        &mut self.val
    }

    /// Returns a pair of references to this `Slot`'s key and value.
    #[inline]
    pub fn get_key_val(&self) -> (&Value<'a>, &Value<'a>) {
        (&self.key, &self.val)
    }

    /// Returns a pair of references to this `Slot`'s key and mutable value.
    #[inline]
    pub fn get_key_val_mut(&mut self) -> (&Value<'a>, &mut Value<'a>) {
        (&self.key, &mut self.val)
    }

    /// Upcasts this `Slot` reference to a `Field` reference.
    #[inline]
    pub fn as_field(&self) -> &Field<'a> {
        unsafe { mem::transmute::<&Slot<'a>, &Field<'a>>(self) }
    }

    /// Upcasts this `Slot` reference to a mutable `Field` reference.
    #[inline]
    pub fn as_mut_field(&mut self) -> &mut Field<'a> {
        unsafe { mem::transmute::<&mut Slot<'a>, &mut Field<'a>>(self) }
    }

    /// Upcasts this `Slot` reference to an `Item` reference.
    #[inline]
    pub fn as_item(&self) -> &Item<'a> {
        unsafe { mem::transmute::<&Slot<'a>, &Item<'a>>(self) }
    }

    /// Upcasts this `Slot` reference to a mutable `Item` reference.
    #[inline]
    pub fn as_mut_item(&mut self) -> &mut Item<'a> {
        unsafe { mem::transmute::<&mut Slot<'a>, &mut Item<'a>>(self) }
    }

    /// Returns this `Slot`'s key, dropping its value.
    #[inline]
    pub fn into_key(mut self) -> Value<'a> {
        let key = unsafe { ptr::read(&self.key) };
        unsafe { ptr::drop_in_place(&mut self.val); }
        mem::forget(self);
        key
    }

    /// Returns this `Slot`'s value, dropping its key.
    #[inline]
    pub fn into_val(mut self) -> Value<'a> {
        unsafe { ptr::drop_in_place(&mut self.key) };
        let val = unsafe { ptr::read(&self.val) };
        mem::forget(self);
        val
    }

    /// Returns this `Slot`'s key and value as a pair.
    #[inline]
    pub fn into_key_val(self) -> (Value<'a>, Value<'a>) {
        let key = unsafe { ptr::read(&self.key) };
        let val = unsafe { ptr::read(&self.val) };
        mem::forget(self);
        (key, val)
    }

    /// Upcasts this `Slot` to a `Field`.
    #[inline]
    pub fn into_field(self) -> Field<'a> {
        unsafe { mem::transmute::<Slot<'a>, Field<'a>>(self) }
    }

    /// Upcasts this `Slot` to an `Item`.
    #[inline]
    pub fn into_item(self) -> Item<'a> {
        unsafe { mem::transmute::<Slot<'a>, Item<'a>>(self) }
    }
}

impl<'a> AsRef<Field<'a>> for Slot<'a> {
    #[inline]
    fn as_ref(&self) -> &Field<'a> {
        self.as_field()
    }
}

impl<'a> AsMut<Field<'a>> for Slot<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Field<'a> {
        self.as_mut_field()
    }
}

impl<'a> AsRef<Item<'a>> for Slot<'a> {
    #[inline]
    fn as_ref(&self) -> &Item<'a> {
        self.as_item()
    }
}

impl<'a> AsMut<Item<'a>> for Slot<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Item<'a> {
        self.as_mut_item()
    }
}

impl<'a> PartialEq for Slot<'a> {
    #[inline]
    fn eq(&self, that: &Slot<'a>) -> bool {
        self.key.eq(&that.key) && self.val.eq(&that.val)
    }
}

impl<'a> hash::Hash for Slot<'a> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.key.hash(hasher);
        self.val.hash(hasher);
    }
}

impl<'a> fmt::Debug for Slot<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("slot")?;
        f.write_char('!')?;
        f.write_char('(')?;
        self.key.fmt(f)?;
        if !self.val.is_extant() {
            f.write_str(", ")?;
            self.val.fmt(f)?;
        }
        f.write_char(')')
    }
}
