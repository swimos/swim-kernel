use core::cmp;
use core::fmt;
use core::hash;
use core::marker::PhantomData;
use core::mem;
use core::num::NonZeroU64;
use core::ops::{Deref, DerefMut, Index, IndexMut, Add, AddAssign};
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use core::ptr;
use core::slice;
use core::str;
use swim_mem::block::{Block, Layout};
use swim_mem::alloc::{Hold, Holder, HoldError, AllocTag, Stow, TryClone, CloneIntoHold};
use swim_mem::lease::PtrString;
use crate::item::{Item, Value};

/// `Value` variant representing a Unicode string.
///
/// # Examples
///
/// Create a `Text` value from a primitive `str`:
///
/// ```
/// # extern crate swim_c_rt;
/// # use swim_structure::item::Text;
/// let value = Text::from_str("Hello");
/// # assert_eq!(value.as_str(), "Hello");
/// ```
///
/// Get a reference to the primitive `str` owned by a `Text` value:
///
/// ```
/// # extern crate swim_c_rt;
/// # use swim_structure::item::Text;
/// let value = Text::from_str("Hello, world!");
/// let primitive = value.as_str();
/// # assert_eq!(primitive, "Hello, world!");
/// ```
///
/// Concatenate a `Text` value with another `str` using the `+` operator:
///
/// ```
/// # extern crate swim_c_rt;
/// # use swim_structure::item::Text;
/// let value = Text::from_str("Hello");
/// let message = value + " " + "world";
/// # assert_eq!(message.len(), 11);
#[derive(Eq, Ord)]
#[repr(C)]
pub struct Text<'a> {
    /// Discriminant with a type between `TEXT_TYPE_MIN` and `TEXT_TYPE_MAX`
    /// at the lowest byte address, and an optional embedded string in the
    /// subsequent 7 bytes.
    ///
    /// ```text
    /// 0        1        2        3        4        5        6        7        8
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// |  type  |     c0 |     c1 |     c2 |     c3 |     c4 |     c5 |     c6 |
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// ```
    _0: NonZeroU64,
    /// Raw pointer to either a PtrString<'a>, or an empty allocation.
    _1: *mut u8,
    /// Variant over allocation lifetime.
    lifetime: PhantomData<PtrString<'a>>,
}

impl<'a> Text<'a> {
    pub fn try_hold_str(hold: &dyn Hold<'a>, data: &str) -> Result<Text<'a>, HoldError> {
        unsafe {
            let len = data.len();
            if len <= Value::TEXT_EMBED_MAX as usize {
                let block = hold.alloc(Layout::empty())?;
                let mut value = Text {
                    _0: NonZeroU64::new_unchecked(Value::discriminant(Value::TEXT0_TYPE + len as u8)),
                    _1: block.into_raw(),
                    lifetime: PhantomData,
                };
                let embed_ptr = (&mut value._0 as *mut NonZeroU64 as *mut u8).wrapping_add(1);
                ptr::copy_nonoverlapping(data.as_ptr(), embed_ptr, len);
                Ok(value)
            } else {
                let string = PtrString::try_hold_copy(hold, data)?;
                Ok(Text {
                    _0: NonZeroU64::new_unchecked(Value::discriminant(Value::TEXT_TYPE)),
                    _1: PtrString::into_raw(string),
                    lifetime: PhantomData,
                })
            }
        }
    }

    pub fn hold_str(hold: &dyn Hold<'a>, data: &str) -> Text<'a> {
        Text::try_hold_str(hold, data).unwrap()
    }

    pub fn from_str(data: &str) -> Text<'a> {
        Text::hold_str(Hold::global(), data)
    }

    /// Returns a pointer to the tag in the first byte of this `Text`.
    #[inline(always)]
    pub(crate) unsafe fn tag_ptr(&self) -> *mut u8 {
        mem::transmute::<&Text<'a>, *mut u8>(self)
    }

    /// Returns the tag from the first byte of this `Text`.
    #[inline(always)]
    pub(crate) fn tag(&self) -> u8 {
        unsafe { *self.tag_ptr() }
    }

    /// Returns the type tag from the low 7 bits of the first byte of this `Text`.
    #[inline(always)]
    pub(crate) fn type_tag(&self) -> u8 {
        self.tag() & Value::TYPE_MASK
    }

    pub fn is_empty(&self) -> bool {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a reference to the string lease.
            let string = unsafe { mem::transmute::<_, &PtrString<'a>>(&self._1) };
            // Return whether or not the string resident is empty.
            string.is_empty()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                // Return whether or not the embedded string is empty.
                Some(len) => len == 0,
                None => unreachable!(),
            }
        }
    }

    pub fn len(&self) -> usize {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a reference to the string lease.
            let string = unsafe { mem::transmute::<_, &PtrString<'a>>(&self._1) };
            // Return the length of the string resident.
            string.len()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                // Return the length of the embedded string.
                Some(len) => len as usize,
                None => unreachable!(),
            }
        }
    }

    pub unsafe fn set_len(&mut self, new_len: usize) {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a mutable reference to the string lease.
            let string = mem::transmute::<_, &mut PtrString<'a>>(&mut self._1);
            // Set the length of the string resident.
            string.set_len(new_len)
        } else {
            assert!((new_len as u8) < Value::TEXT_EMBED_MAX);
            // Make a discriminant tag with the new embedded string length.
            let tag = Value::TEXT0_TYPE.wrapping_add(new_len as u8);
            // Splice the updated tag into the discriminant.
            let discriminant = self._0.get() & !Value::discriminant(Value::TYPE_MASK) | Value::discriminant(tag);
            // Write the new discriminant.
            ptr::write(&mut self._0, NonZeroU64::new_unchecked(discriminant));
        }
    }

    pub fn cap(&self) -> usize {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a reference to the string lease.
            let string = unsafe { mem::transmute::<_, &PtrString<'a>>(&self._1) };
            // Return the capacity of the string resident.
            string.cap()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                // Return the capacity of the embedded string.
                Some(_) => Value::TEXT_EMBED_MAX as usize,
                None => unreachable!(),
            }
        }
    }

    /// Upcasts this `Text` reference to a `Value` reference.
    #[inline]
    pub fn as_value(&self) -> &Value<'a> {
        unsafe { mem::transmute::<&Text<'a>, &Value<'a>>(self) }
    }

    /// Upcasts this `Text` reference to a mutable `Value` reference.
    #[inline]
    pub fn as_mut_value(&mut self) -> &mut Value<'a> {
        unsafe { mem::transmute::<&mut Text<'a>, &mut Value<'a>>(self) }
    }

    /// Upcasts this `Text` to a `Value`.
    #[inline]
    pub fn into_value(self) -> Value<'a> {
        unsafe { mem::transmute::<Text<'a>, Value<'a>>(self) }
    }

    /// Upcasts this `Text` to an `Item`.
    #[inline]
    pub fn into_item(self) -> Item<'a> {
        Item::from_value(self.into_value())
    }

    pub fn as_ptr(&self) -> *const u8 {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a reference to the string lease.
            let string = unsafe { mem::transmute::<_, &PtrString<'a>>(&self._1) };
            // Return a pointer to the resident data.
            string.as_ptr()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                // Return a pointer to the embedded string.
                Some(_) => (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1),
                None => unreachable!(),
            }
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a mutable reference to the string lease.
            let string = unsafe { mem::transmute::<_, &mut PtrString<'a>>(&mut self._1) };
            // Return a mutable pointer to the resident data.
            string.as_mut_ptr()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                // Return a mutable pointer to the embedded string.
                Some(_) => (&mut self._0 as *mut NonZeroU64 as *mut u8).wrapping_add(1),
                None => unreachable!(),
            }
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a reference to the string lease.
            let string = unsafe { mem::transmute::<_, &PtrString<'a>>(&self._1) };
            // Return the resident slice.
            string.as_slice()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                Some(len) => {
                    // Get a pointer to the embedded string.
                    let embed_ptr = (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1);
                    // Return the embedded slice.
                    unsafe { slice::from_raw_parts(embed_ptr, len as usize) }
                },
                None => unreachable!(),
            }
        }
    }

    pub fn as_str(&self) -> &str {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a reference to the string lease.
            let string = unsafe { mem::transmute::<_, &PtrString<'a>>(&self._1) };
            // Return the resident string.
            string.as_str()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                Some(len) => {
                    // Get a pointer to the embedded string.
                    let embed_ptr = (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1);
                    // Return the embedded string.
                    unsafe { str::from_utf8_unchecked(slice::from_raw_parts(embed_ptr, len as usize)) }
                },
                None => unreachable!(),
            }
        }
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a reference to the string lease.
            let string = unsafe { mem::transmute::<_, &mut PtrString<'a>>(&mut self._1) };
            // Return the resident string.
            string.as_mut_str()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                Some(len) => {
                    // Get a pointer to the embedded string.
                    let embed_ptr = (&mut self._0 as *mut NonZeroU64 as *mut u8).wrapping_add(1);
                    // Return the embedded string.
                    unsafe { str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(embed_ptr, len as usize)) }
                },
                None => unreachable!(),
            }
        }
    }

    pub fn try_reserve(&mut self, ext: usize) -> Result<(), HoldError> {
        unsafe {
            let tag = self.tag();
            let type_tag = tag & Value::TYPE_MASK;
            if type_tag == Value::TEXT_TYPE {
                // Reconstitute a reference to the string lease.
                let string = mem::transmute::<_, &mut PtrString<'a>>(&mut self._1);
                // Reserve capacity in the string.
                string.try_reserve(ext)
            } else {
                match type_tag.checked_sub(Value::TEXT0_TYPE) {
                    Some(len) => {
                        if (Value::TEXT_EMBED_MAX.wrapping_sub(len) as usize) < ext {
                            // Get a pointer to the embedded string.
                            let embed_ptr = (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1);
                            // Reconstruct the placeholder allocation block.
                            let block = Block::from_raw_parts(self._1 as *mut u8, 0);
                            // Get a reference to the hold that allocates this value.
                            let hold = AllocTag::from_ptr(self._1 as *mut u8).holder();
                            // Allocate a new string in the hold.
                            let mut string = PtrString::try_hold_cap(hold, len as usize + ext)?;
                            // Copy the embedded slice into the new string.
                            ptr::copy_nonoverlapping(embed_ptr, string.as_mut_ptr(), len as usize);
                            // Set the length of the new string.
                            string.set_len(len as usize);
                            // Modify the discriminant tag with the new record type.
                            let tag = tag & Value::ATTR_FLAG | Value::TEXT_TYPE;
                            // Write the new discriminant.
                            ptr::write(&mut self._0, NonZeroU64::new_unchecked(Value::discriminant(tag)));
                            // Write the new data pointer.
                            ptr::write(&mut self._1, PtrString::into_raw(string));
                            // Deallocate the placeholder block.
                            hold.dealloc(block);
                        }
                        Ok(())
                    },
                    None => unreachable!(),
                }
            }
        }
    }

    pub fn reserve(&mut self, ext: usize) {
        self.try_reserve(ext).unwrap();
    }

    pub fn try_reserve_exact(&mut self, ext: usize) -> Result<(), HoldError> {
        unsafe {
            let tag = self.tag();
            let type_tag = tag & Value::TYPE_MASK;
            if type_tag == Value::TEXT_TYPE {
                // Reconstitute a reference to the string lease.
                let string = mem::transmute::<_, &mut PtrString<'a>>(&mut self._1);
                // Reserve capacity in the string.
                string.try_reserve_exact(ext)
            } else {
                match type_tag.checked_sub(Value::TEXT0_TYPE) {
                    Some(len) => {
                        if (Value::TEXT_EMBED_MAX.wrapping_sub(len) as usize) < ext {
                            // Get a pointer to the embedded string.
                            let embed_ptr = (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1);
                            // Reconstruct the placeholder allocation block.
                            let block = Block::from_raw_parts(self._1 as *mut u8, 0);
                            // Get a reference to the hold that allocates this value.
                            let hold = AllocTag::from_ptr(self._1 as *mut u8).holder();
                            // Allocate a new string in the hold.
                            let mut string = PtrString::try_hold_cap(hold, len as usize + ext)?;
                            // Copy the embedded slice into the new string.
                            ptr::copy_nonoverlapping(embed_ptr, string.as_mut_ptr(), len as usize);
                            // Set the length of the new string.
                            string.set_len(len as usize);
                            // Modify the discriminant tag with the new record type.
                            let tag = tag & Value::ATTR_FLAG | Value::TEXT_TYPE;
                            // Write the new discriminant.
                            ptr::write(&mut self._0, NonZeroU64::new_unchecked(Value::discriminant(tag)));
                            // Write the new data pointer.
                            ptr::write(&mut self._1, PtrString::into_raw(string));
                            // Deallocate the placeholder block.
                            hold.dealloc(block);
                        }
                        Ok(())
                    },
                    None => unreachable!(),
                }
            }
        }
    }

    pub fn reserve_exact(&mut self, ext: usize) {
        self.try_reserve_exact(ext).unwrap();
    }

    pub fn try_reserve_in_place(&mut self, ext: usize) -> Result<(), HoldError> {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag == Value::TEXT_TYPE {
                // Reconstitute a reference to the string lease.
                let string = mem::transmute::<_, &mut PtrString<'a>>(&mut self._1);
                // Reserve capacity in the string.
                string.try_reserve_in_place(ext)
            } else {
                match type_tag.checked_sub(Value::TEXT0_TYPE) {
                    Some(len) => {
                        if (Value::TEXT_EMBED_MAX.wrapping_sub(len) as usize) >= ext {
                            Ok(())
                        } else {
                            Err(HoldError::Oversized)
                        }
                    },
                    None => unreachable!(),
                }
            }
        }
    }

    pub fn try_reserve_in_place_exact(&mut self, ext: usize) -> Result<(), HoldError> {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag == Value::TEXT_TYPE {
                // Reconstitute a reference to the string lease.
                let string = mem::transmute::<_, &mut PtrString<'a>>(&mut self._1);
                // Reserve capacity in the string.
                string.try_reserve_in_place_exact(ext)
            } else {
                match type_tag.checked_sub(Value::TEXT0_TYPE) {
                    Some(len) => {
                        if (Value::TEXT_EMBED_MAX.wrapping_sub(len) as usize) >= ext {
                            Ok(())
                        } else {
                            Err(HoldError::Oversized)
                        }
                    },
                    None => unreachable!(),
                }
            }
        }
    }

    pub fn try_push(&mut self, c: char) -> Result<(), HoldError> {
        unsafe {
            let mut bytes = [0u8; 4];
            let n = c.encode_utf8(&mut bytes).len();
            self.try_reserve(n)?;
            let len = self.len();
            let data = self.as_mut_ptr().wrapping_add(len);
            ptr::copy_nonoverlapping(bytes.as_ptr(), data, n);
            self.set_len(len.wrapping_add(n));
            Ok(())
        }
    }

    pub fn push(&mut self, c: char) {
        self.try_push(c).unwrap();
    }

    pub fn try_push_str(&mut self, s: &str) -> Result<(), HoldError> {
        unsafe {
            let n = s.len();
            self.try_reserve(n)?;
            let len = self.len();
            let data = self.as_mut_ptr().wrapping_add(len);
            ptr::copy_nonoverlapping(s.as_ptr(), data, n);
            self.set_len(len.wrapping_add(n));
            Ok(())
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.try_push_str(s).unwrap();
    }

    pub fn try_insert(&mut self, index: usize, c: char) -> Result<(), HoldError> {
        unsafe {
            assert!(self.as_str().is_char_boundary(index));
            let mut slice = [0u8; 4];
            let slice = c.encode_utf8(&mut slice).as_bytes();
            self.try_insert_slice(index, slice)
        }
    }

    pub fn insert(&mut self, index: usize, c: char) {
        self.try_insert(index, c).unwrap();
    }

    pub fn try_insert_str(&mut self, index: usize, s: &str) -> Result<(), HoldError> {
        unsafe {
            assert!(self.as_str().is_char_boundary(index));
            self.try_insert_slice(index, s.as_bytes())
        }
    }

    pub fn insert_str(&mut self, index: usize, s: &str) {
        self.try_insert_str(index, s).unwrap();
    }

    pub unsafe fn try_insert_slice(&mut self, index: usize, slice: &[u8]) -> Result<(), HoldError> {
        let n = slice.len();
        self.try_reserve(n)?;
        let len = self.len();
        let data = self.as_mut_ptr();
        ptr::copy(data.wrapping_add(index),
                  data.wrapping_add(index.wrapping_add(n)),
                  len.wrapping_sub(index));
        ptr::copy(slice.as_ptr(),
                  data.wrapping_add(index),
                  n);
        self.set_len(len.wrapping_add(n));
        Ok(())
    }

    pub unsafe fn insert_slice(&mut self, index: usize, slice: &[u8]) {
        self.try_insert_slice(index, slice).unwrap();
    }

    pub fn pop(&mut self) -> Option<char> {
        unsafe {
            let c = self.as_str().chars().rev().next()?;
            self.set_len(self.len().wrapping_sub(c.len_utf8()));
            Some(c)
        }
    }

    pub fn remove(&mut self, index: usize) -> char {
        unsafe {
            let c = self.as_str()[index..].chars().next().unwrap();
            let n = c.len_utf8();
            let next = index.wrapping_add(n);
            let len = self.len();
            let data = self.as_mut_ptr();
            ptr::copy(data.wrapping_add(next),
                      data.wrapping_add(index),
                      len.wrapping_sub(next));
            self.set_len(len.wrapping_sub(n));
            c
        }
    }

    pub fn clear(&mut self) {
        unsafe { self.set_len(0); }
    }

    pub(crate) unsafe fn dealloc(&mut self) {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconsitute the Hold-allocated string.
            let string = PtrString::<'a, ()>::from_raw(self._1);
            // And drop it.
            mem::drop(string);
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                Some(_) => {
                    // Reconstruct the empty allocation block.
                    let block = Block::from_raw_parts(self._1, 0);
                    // Get a pointer to the AllocTag of the block,
                    // and use it to deallocate the empty block.
                    AllocTag::from_ptr(self._1).dealloc(block);
                },
                None => unreachable!(),
            }
        }
    }
}

impl<'a> Holder<'a> for Text<'a> {
    #[inline]
    fn holder(&self) -> &'a dyn Hold<'a> {
        let type_tag = self.type_tag();
        if type_tag == Value::TEXT_TYPE {
            // Reconstitute a reference to the string lease.
            let string = unsafe { mem::transmute::<_, &PtrString<'a>>(&self._1) };
            // Return the string resident holder.
            string.holder()
        } else {
            match type_tag.checked_sub(Value::TEXT0_TYPE) {
                // Return the embedded string holder.
                Some(_) => AllocTag::from_ptr(self._1).holder(),
                None => unreachable!(),
            }
        }
    }
}

impl<'a> Deref for Text<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> DerefMut for Text<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<'a> AsRef<Value<'a>> for Text<'a> {
    #[inline]
    fn as_ref(&self) -> &Value<'a> {
        self.as_value()
    }
}

impl<'a> AsRef<str> for Text<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> AsRef<[u8]> for Text<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> AsMut<Value<'a>> for Text<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Value<'a> {
        self.as_mut_value()
    }
}

impl<'a> AsMut<str> for Text<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<'a> Index<Range<usize>> for Text<'a> {
    type Output = str;

    #[inline]
    fn index(&self, index: Range<usize>) -> &str {
        self.as_str().index(index)
    }
}

impl<'a> Index<RangeFrom<usize>> for Text<'a> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &str {
        self.as_str().index(index)
    }
}

impl<'a> Index<RangeFull> for Text<'a> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFull) -> &str {
        self.as_str().index(index)
    }
}

impl<'a> Index<RangeInclusive<usize>> for Text<'a> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &str {
        self.as_str().index(index)
    }
}

impl<'a> Index<RangeTo<usize>> for Text<'a> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &str {
        self.as_str().index(index)
    }
}

impl<'a> Index<RangeToInclusive<usize>> for Text<'a> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &str {
        self.as_str().index(index)
    }
}

impl<'a> IndexMut<Range<usize>> for Text<'a> {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut str {
        self.as_mut_str().index_mut(index)
    }
}

impl<'a> IndexMut<RangeFrom<usize>> for Text<'a> {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut str {
        self.as_mut_str().index_mut(index)
    }
}

impl<'a> IndexMut<RangeFull> for Text<'a> {
    #[inline]
    fn index_mut(&mut self, index: RangeFull) -> &mut str {
        self.as_mut_str().index_mut(index)
    }
}

impl<'a> IndexMut<RangeInclusive<usize>> for Text<'a> {
    #[inline]
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut str {
        self.as_mut_str().index_mut(index)
    }
}

impl<'a> IndexMut<RangeTo<usize>> for Text<'a> {
    #[inline]
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut str {
        self.as_mut_str().index_mut(index)
    }
}

impl<'a> IndexMut<RangeToInclusive<usize>> for Text<'a> {
    #[inline]
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut str {
        self.as_mut_str().index_mut(index)
    }
}

impl<'a, 'b> Add<&'b str> for Text<'a> {
    type Output = Text<'a>;

    #[inline]
    fn add(mut self, rhs: &'b str) -> Text<'a> {
        self.push_str(rhs);
        self
    }
}

impl<'a, 'b> AddAssign<&'b str> for Text<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: &'b str) {
        self.push_str(rhs);
    }
}

impl<'a> PartialEq for Text<'a> {
    #[inline]
    fn eq(&self, that: &Text<'a>) -> bool {
        self.as_str().eq(that.as_str())
    }

    #[inline]
    fn ne(&self, that: &Text<'a>) -> bool {
        self.as_str().ne(that.as_str())
    }
}

impl<'a> cmp::PartialOrd<Text<'a>> for Text<'a> {
    #[inline]
    fn partial_cmp(&self, that: &Text<'a>) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(that.as_str())
    }

    #[inline]
    fn lt(&self, that: &Text<'a>) -> bool {
        self.as_str().lt(that.as_str())
    }

    #[inline]
    fn le(&self, that: &Text<'a>) -> bool {
        self.as_str().le(that.as_str())
    }

    #[inline]
    fn ge(&self, that: &Text<'a>) -> bool {
        self.as_str().ge(that.as_str())
    }

    #[inline]
    fn gt(&self, that: &Text<'a>) -> bool {
        self.as_str().gt(that.as_str())
    }
}

impl<'a> hash::Hash for Text<'a> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher);
    }
}

impl<'a> fmt::Display for Text<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<'a> fmt::Debug for Text<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!(); // TODO
    }
}

impl<'a> Clone for Text<'a> {
    fn clone(&self) -> Text<'a> {
        Text::from_str(self.as_str())
    }
}

impl<'a> TryClone for Text<'a> {
    fn try_clone(&self) -> Result<Text<'a>, HoldError> {
        self.try_clone_into_hold(self.holder())
    }
}

impl<'a, 'b> CloneIntoHold<'a, Text<'a>> for Text<'b> {
    fn try_clone_into_hold(&self, hold: &Hold<'a>) -> Result<Text<'a>, HoldError> {
        Text::try_hold_str(hold, self.as_str())
    }
}

impl<'a, 'b> Stow<'b, Text<'b>> for Text<'a> {
    unsafe fn stow(src: *mut Text<'a>, dst: *mut Text<'b>, hold: &Hold<'b>) -> Result<(), HoldError> {
        let len = (*src).len();
        if len <= Value::TEXT_EMBED_MAX as usize {
            let block = hold.alloc(Layout::empty())?;
            ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(Value::TEXT0_TYPE + len as u8)));
            ptr::write(&mut (*dst)._1, block.into_raw());
            let embed_ptr = (&mut (*dst)._0 as *mut NonZeroU64 as *mut u8).wrapping_add(1);
            ptr::copy_nonoverlapping((*src).as_ptr(), embed_ptr, len);
        } else {
            let string = PtrString::try_hold_copy(hold, (*src).as_str())?;
            ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(Value::TEXT_TYPE)));
            ptr::write(&mut (*dst)._1, PtrString::into_raw(string));
        }
        Ok(())
    }

    unsafe fn unstow(_src: *mut Text<'a>, _dst: *mut Text<'b>) {
        unimplemented!();
    }
}

impl<'a> Drop for Text<'a> {
    fn drop(&mut self) {
        unsafe { self.dealloc(); }
    }
}
