use core::fmt::{self, Write};
use core::hash;
use core::mem;
use core::ptr;
use crate::item::{Item, Field, Value, Text};

/// `Field` variant representing a key-value attribute with a `Text` key and a
/// `Value`-typed value.
///
/// # Examples
///
/// Construct a new `Attr` field:
///
/// ```
/// # extern crate swim_c_rt;
/// # use swim_structure::item::{Attr, Value, Text};
/// let field = Attr::new(Text::from_str("a"), Value::from(true));
/// # assert!(field.as_item().is_attr());
/// # assert_eq!(field.get_key().as_str(), "a");
/// # assert_eq!(field.get_val().as_bool().to_bool(), true);
/// ```
#[derive(Eq)]
#[repr(C)]
pub struct Attr<'a> {
    val: Value<'a>,
    key: Text<'a>,
}

impl<'a> Attr<'a> {
    pub fn new(key: Text<'a>, val: Value<'a>) -> Attr<'a> {
        let attr = Attr {
            val: val,
            key: key,
        };
        unsafe {
            let key_tag_ptr = attr.key.tag_ptr();
            ptr::write(key_tag_ptr, *key_tag_ptr | Value::ATTR_FLAG);
        }
        attr
    }

    /// Returns a reference to this `Attr`'s key.
    #[inline]
    pub fn get_key(&self) -> &Text<'a> {
        &self.key
    }

    /// Returns a reference to this `Attr`'s value.
    #[inline]
    pub fn get_val(&self) -> &Value<'a> {
        &self.val
    }

    /// Returns a mutable reference to this `Attr`'s value.
    #[inline]
    pub fn get_val_mut(&mut self) -> &mut Value<'a> {
        &mut self.val
    }

    /// Returns a pair of references to this `Attr`'s key and value.
    #[inline]
    pub fn get_key_val(&self) -> (&Text<'a>, &Value<'a>) {
        (&self.key, &self.val)
    }

    /// Returns a pair of references to this `Attr`'s key and mutable value.
    #[inline]
    pub fn get_key_val_mut(&mut self) -> (&Text<'a>, &mut Value<'a>) {
        (&self.key, &mut self.val)
    }

    /// Upcasts this `Attr` reference to a `Field` reference.
    #[inline]
    pub fn as_field(&self) -> &Field<'a> {
        unsafe { mem::transmute::<&Attr<'a>, &Field<'a>>(self) }
    }

    /// Upcasts this `Attr` reference to a mutable `Field` reference.
    #[inline]
    pub fn as_mut_field(&mut self) -> &mut Field<'a> {
        unsafe { mem::transmute::<&mut Attr<'a>, &mut Field<'a>>(self) }
    }

    /// Upcasts this `Attr` reference to an `Item` reference.
    #[inline]
    pub fn as_item(&self) -> &Item<'a> {
        unsafe { mem::transmute::<&Attr<'a>, &Item<'a>>(self) }
    }

    /// Upcasts this `Attr` reference to a mutable `Item` reference.
    #[inline]
    pub fn as_mut_item(&mut self) -> &mut Item<'a> {
        unsafe { mem::transmute::<&mut Attr<'a>, &mut Item<'a>>(self) }
    }

    /// Returns this `Attr`'s key, dropping its value.
    #[inline]
    pub fn into_key(mut self) -> Text<'a> {
        let key = unsafe { ptr::read(&self.key) };
        unsafe { ptr::drop_in_place(&mut self.val); }
        mem::forget(self);
        key
    }

    /// Returns this `Attr`'s value, dropping its key.
    #[inline]
    pub fn into_val(mut self) -> Value<'a> {
        unsafe { ptr::drop_in_place(&mut self.key) };
        let val = unsafe { ptr::read(&self.val) };
        mem::forget(self);
        val
    }

    /// Returns this `Attr`'s key and value as a pair.
    #[inline]
    pub fn into_key_val(self) -> (Text<'a>, Value<'a>) {
        let key = unsafe { ptr::read(&self.key) };
        let val = unsafe { ptr::read(&self.val) };
        mem::forget(self);
        (key, val)
    }

    /// Upcasts this `Attr` to a `Field`.
    #[inline]
    pub fn into_field(self) -> Field<'a> {
        unsafe { mem::transmute::<Attr<'a>, Field<'a>>(self) }
    }

    /// Upcasts this `Attr` to an `Item`.
    #[inline]
    pub fn into_item(self) -> Item<'a> {
        unsafe { mem::transmute::<Attr<'a>, Item<'a>>(self) }
    }
}

impl<'a> AsRef<Field<'a>> for Attr<'a> {
    #[inline]
    fn as_ref(&self) -> &Field<'a> {
        self.as_field()
    }
}

impl<'a> AsMut<Field<'a>> for Attr<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Field<'a> {
        self.as_mut_field()
    }
}

impl<'a> AsRef<Item<'a>> for Attr<'a> {
    #[inline]
    fn as_ref(&self) -> &Item<'a> {
        self.as_item()
    }
}

impl<'a> AsMut<Item<'a>> for Attr<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Item<'a> {
        self.as_mut_item()
    }
}

impl<'a> PartialEq for Attr<'a> {
    #[inline]
    fn eq(&self, that: &Attr<'a>) -> bool {
        self.key.eq(&that.key) && self.val.eq(&that.val)
    }
}

impl<'a> hash::Hash for Attr<'a> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.key.hash(hasher);
        self.val.hash(hasher);
    }
}

impl<'a> fmt::Debug for Attr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("attr")?;
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
