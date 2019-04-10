use core::fmt;
use core::hash;
use core::mem;
use core::num::NonZeroU64;
use core::ptr;
use swim_mem::alloc::{Hold, HoldError, Stow, TryClone, CloneIntoHold};
use crate::item::{Item, Value};

/// `Value` variant representing an undefined value.
#[derive(Eq)]
#[repr(C)]
pub struct Absent {
    /// Discriminant with `ABSENT_TYPE` at the lowest byte address.
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

impl Absent {
    /// Constructs a new `Absent` value.
    pub const fn new() -> Absent {
        Absent {
            _0: unsafe { NonZeroU64::new_unchecked(Value::discriminant(Value::ABSENT_TYPE)) },
            _1: 0,
        }
    }

    /// Upcasts this `Absent` reference to a `Value` reference.
    #[inline]
    pub fn as_value<'a>(&self) -> &Value<'a> {
        unsafe { mem::transmute::<&Absent, &Value<'a>>(self) }
    }

    /// Upcasts this `Absent` reference to a mutable `Value` reference.
    #[inline]
    pub fn as_mut_value<'a>(&mut self) -> &mut Value<'a> {
        unsafe { mem::transmute::<&mut Absent, &mut Value<'a>>(self) }
    }

    /// Upcasts `Absent` to a `Value`.
    #[inline]
    pub fn into_value<'a>(self) -> Value<'a> {
        unsafe { mem::transmute::<Absent, Value<'a>>(self) }
    }

    /// Upcasts `Absent` to an `Item`.
    #[inline]
    pub fn into_item<'a>(self) -> Item<'a> {
        Item::from_value(self.into_value())
    }
}

impl<'a> AsRef<Value<'a>> for Absent {
    #[inline]
    fn as_ref(&self) -> &Value<'a> {
        self.as_value()
    }
}

impl<'a> AsMut<Value<'a>> for Absent {
    #[inline]
    fn as_mut(&mut self) -> &mut Value<'a> {
        self.as_mut_value()
    }
}

impl PartialEq for Absent {
    #[inline]
    fn eq(&self, _that: &Absent) -> bool {
        true
    }
}

impl hash::Hash for Absent {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        0.hash(hasher);
    }
}

impl fmt::Debug for Absent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Absent")
    }
}

impl Clone for Absent {
    fn clone(&self) -> Absent {
        Absent::new()
    }
}

impl TryClone for Absent {
    fn try_clone(&self) -> Result<Absent, HoldError> {
        Ok(Absent::new())
    }
}

impl<'a> CloneIntoHold<'a, Absent> for Absent {
    fn try_clone_into_hold(&self, _hold: &Hold<'a>) -> Result<Absent, HoldError> {
        Ok(Absent::new())
    }
}

impl<'b> Stow<'b, Absent> for Absent {
    unsafe fn stow(src: *mut Absent, dst: *mut Absent, _hold: &Hold<'b>) -> Result<(), HoldError> {
        ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(Value::ABSENT_TYPE)));
        ptr::write(&mut (*dst)._1, (*src)._1);
        Ok(())
    }

    unsafe fn unstow(_src: *mut Absent, _dst: *mut Absent) {
        // nop
    }
}

