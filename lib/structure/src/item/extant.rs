use core::fmt;
use core::hash;
use core::mem;
use core::num::NonZeroU64;
use core::ptr;
use swim_mem::alloc::{Hold, HoldError, Stow, TryClone, CloneIntoHold};
use crate::item::{Item, Value};

/// `Value` variant representing a defined—but unspecified—value.
#[derive(Eq)]
#[repr(C)]
pub struct Extant {
    /// Discriminant with `EXTANT_TYPE` at the lowest byte address.
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

impl Extant {
    /// Constructs a new `Extant` value.
    pub const fn new() -> Extant {
        Extant {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::EXTANT_TYPE)) },
            _1: 0,
        }
    }

    /// Upcasts this `Extant` reference to a `Value` reference.
    #[inline]
    pub fn as_value<'a>(&self) -> &Value<'a> {
        unsafe { mem::transmute::<&Extant, &Value<'a>>(self) }
    }

    /// Upcasts this `Extant` reference to a mutable `Value` reference.
    #[inline]
    pub fn as_mut_value<'a>(&mut self) -> &mut Value<'a> {
        unsafe { mem::transmute::<&mut Extant, &mut Value<'a>>(self) }
    }

    /// Upcasts `Extant` to a `Value`.
    #[inline]
    pub fn into_value<'a>(self) -> Value<'a> {
        unsafe { mem::transmute::<Extant, Value<'a>>(self) }
    }

    /// Upcasts `Extant` to an `Item`.
    #[inline]
    pub fn into_item<'a>(self) -> Item<'a> {
        Item::from_value(self.into_value())
    }
}

impl<'a> AsRef<Value<'a>> for Extant {
    #[inline]
    fn as_ref(&self) -> &Value<'a> {
        self.as_value()
    }
}

impl<'a> AsMut<Value<'a>> for Extant {
    #[inline]
    fn as_mut(&mut self) -> &mut Value<'a> {
        self.as_mut_value()
    }
}

impl PartialEq for Extant {
    #[inline]
    fn eq(&self, _that: &Extant) -> bool {
        true
    }
}

impl hash::Hash for Extant {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        1.hash(hasher);
    }
}

impl fmt::Debug for Extant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Extant")
    }
}

impl Clone for Extant {
    fn clone(&self) -> Extant {
        Extant::new()
    }
}

impl TryClone for Extant {
    fn try_clone(&self) -> Result<Extant, HoldError> {
        Ok(Extant::new())
    }
}

impl<'a> CloneIntoHold<'a, Extant> for Extant {
    fn try_clone_into_hold(&self, _hold: &Hold<'a>) -> Result<Extant, HoldError> {
        Ok(Extant::new())
    }
}

impl<'b> Stow<'b, Extant> for Extant {
    unsafe fn stow(src: *mut Extant, dst: *mut Extant, _hold: &Hold<'b>) -> Result<(), HoldError> {
        ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(Value::EXTANT_TYPE)));
        ptr::write(&mut (*dst)._1, (*src)._1);
        Ok(())
    }

    unsafe fn unstow(_src: *mut Extant, _dst: *mut Extant) {
        // nop
    }
}
