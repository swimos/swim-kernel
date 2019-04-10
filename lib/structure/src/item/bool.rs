use core::fmt;
use core::hash;
use core::mem;
use core::num::NonZeroU64;
use core::ptr;
use swim_mem::alloc::{Hold, HoldError, Stow, TryClone, CloneIntoHold};
use crate::item::{Item, Value};

/// `Value` variant representing a boolean.
///
/// # Examples
///
/// Convert a primitive `bool` into a structurally typed `Bool` value:
///
/// ```
/// # use swim_structure::item::Bool;
/// let value = Bool::from(true);
/// # assert_eq!(value.to_bool(), true);
/// ```
///
/// Extract a primitive `bool` out of a structurally typed `Bool` value:
///
/// ```
/// # use swim_structure::item::Bool;
/// let value = Bool::from(false);
/// let primitive = value.to_bool();
/// # assert_eq!(primitive, false);
/// ```
#[derive(Eq)]
#[repr(C)]
pub struct Bool {
    /// Discriminant with `TRUE_TYPE` or `FALSE_TYPE` at the lowest byte address.
    ///
    /// ```text
    /// 0        1        2        3        4        5        6        7        8
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// |  type  |      0 |      0 |      0 |      0 |      0 |      0 |      0 |
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// ```
    _0: NonZeroU64,
    /// Reserved.
    _1: u64,
}

impl Bool {
    /// Constructs a new `Bool` from a `bool` value.
    pub const fn from_bool(value: bool) -> Bool {
        Bool {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::FALSE_TYPE + value as u8)) },
            _1: value as u64,
        }
    }

    /// Returns a pointer to the tag in the first byte of this `Bool`.
    #[inline(always)]
    pub(crate) unsafe fn tag_ptr(&self) -> *mut u8 {
        mem::transmute::<&Bool, *mut u8>(self)
    }

    /// Returns the tag from the first byte of this `Bool`.
    #[inline(always)]
    pub(crate) fn tag(&self) -> u8 {
        unsafe { *self.tag_ptr() }
    }

    /// Returns the type tag from the low 7 bits of the first byte of this `Bool`.
    #[inline(always)]
    pub(crate) fn type_tag(&self) -> u8 {
        self.tag() & Value::TYPE_MASK
    }

    /// Returns the `bool` value of this `Bool` reference.
    pub fn to_bool(&self) -> bool {
        self.type_tag() != Value::FALSE_TYPE
    }

    /// Upcasts this `Bool` reference to a `Value` reference.
    #[inline]
    pub fn as_value<'a>(&self) -> &Value<'a> {
        unsafe { mem::transmute::<&Bool, &Value<'a>>(self) }
    }

    /// Upcasts this `Bool` reference to a mutable `Value` reference.
    #[inline]
    pub fn as_mut_value<'a>(&mut self) -> &mut Value<'a> {
        unsafe { mem::transmute::<&mut Bool, &mut Value<'a>>(self) }
    }

    /// Upcasts this `Bool` to a `Value`.
    #[inline]
    pub fn into_value<'a>(self) -> Value<'a> {
        unsafe { mem::transmute::<Bool, Value<'a>>(self) }
    }

    /// Upcasts this `Bool` to an `Item`.
    #[inline]
    pub fn into_item<'a>(self) -> Item<'a> {
        Item::from_value(self.into_value())
    }
}

impl<'a> AsRef<Value<'a>> for Bool {
    #[inline]
    fn as_ref(&self) -> &Value<'a> {
        self.as_value()
    }
}

impl<'a> AsMut<Value<'a>> for Bool {
    #[inline]
    fn as_mut(&mut self) -> &mut Value<'a> {
        self.as_mut_value()
    }
}

impl PartialEq for Bool {
    #[inline]
    fn eq(&self, that: &Bool) -> bool {
        self.to_bool() == that.to_bool()
    }
}

impl hash::Hash for Bool {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.to_bool().hash(hasher)
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(if self.to_bool() { "True" } else { "False" })
    }
}

impl Clone for Bool {
    fn clone(&self) -> Bool {
        Bool::from_bool(self.to_bool())
    }
}

impl TryClone for Bool {
    fn try_clone(&self) -> Result<Bool, HoldError> {
        Ok(Bool::from_bool(self.to_bool()))
    }
}

impl<'a> CloneIntoHold<'a, Bool> for Bool {
    fn try_clone_into_hold(&self, _hold: &Hold<'a>) -> Result<Bool, HoldError> {
        Ok(Bool::from_bool(self.to_bool()))
    }
}

impl<'b> Stow<'b, Bool> for Bool {
    unsafe fn stow(src: *mut Bool, dst: *mut Bool, _hold: &Hold<'b>) -> Result<(), HoldError> {
        ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant((*src).type_tag())));
        ptr::write(&mut (*dst)._1, (*src)._1);
        Ok(())
    }

    unsafe fn unstow(_src: *mut Bool, _dst: *mut Bool) {
        // nop
    }
}

impl Default for Bool {
    #[inline]
    fn default() -> Bool {
        Bool::from_bool(false)
    }
}

impl From<bool> for Bool {
    #[inline]
    fn from(value: bool) -> Bool {
        Bool::from_bool(value)
    }
}

impl From<Bool> for bool {
    #[inline]
    fn from(value: Bool) -> bool {
        value.to_bool()
    }
}
