use core::cmp;
use core::fmt;
use core::hash;
use core::marker::PhantomData;
use core::mem;
use core::num::NonZeroU64;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr;
use core::slice::{self, SliceIndex};
use swim_mem::block::{Block, Layout};
use swim_mem::alloc::{Hold, Holder, HoldError, AllocTag, Stow, TryClone, CloneIntoHold};
use swim_mem::lease::PtrBuf;
use crate::item::{Item, Value};

/// `Value` variant representing a byte buffer.
#[derive(Eq, Ord)]
#[repr(C)]
pub struct Data<'a> {
    /// Discriminant with a type between `DATA_TYPE_MIN` and `DATA_TYPE_MAX`
    /// at the lowest byte address, and an optional embedded buffer in the
    /// subsequent 7 bytes.
    ///
    /// ```text
    /// 0        1        2        3        4        5        6        7        8
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// |  type  |     x0 |     x1 |     x2 |     x3 |     x4 |     x5 |     x6 |
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// ```
    _0: NonZeroU64,
    /// Raw pointer to either a PtrBuf<'a, u8>, or an empty allocation.
    _1: *mut u8,
    /// Variant over allocation lifetime.
    lifetime: PhantomData<PtrBuf<'a, u8>>,
}

impl<'a> Data<'a> {
    pub fn try_hold_slice(hold: &dyn Hold<'a>, data: &[u8]) -> Result<Data<'a>, HoldError> {
        unsafe {
            let len = data.len();
            if len <= Value::DATA_EMBED_MAX as usize {
                let block = hold.alloc(Layout::empty())?;
                let mut value = Data {
                    _0: NonZeroU64::new_unchecked(Value::discriminant(Value::DATA0_TYPE + len as u8)),
                    _1: block.into_raw(),
                    lifetime: PhantomData,
                };
                let embed_ptr = (&mut value._0 as *mut NonZeroU64 as *mut u8).wrapping_add(1);
                ptr::copy_nonoverlapping(data.as_ptr(), embed_ptr, len);
                Ok(value)
            } else {
                let buf = PtrBuf::try_hold_copy(hold, data)?;
                Ok(Data {
                    _0: NonZeroU64::new_unchecked(Value::discriminant(Value::DATA_TYPE)),
                    _1: PtrBuf::into_raw(buf),
                    lifetime: PhantomData,
                })
            }
        }
    }

    pub fn hold_slice(hold: &dyn Hold<'a>, data: &[u8]) -> Data<'a> {
        Data::try_hold_slice(hold, data).unwrap()
    }

    pub fn from_slice(data: &[u8]) -> Data<'a> {
        Data::hold_slice(Hold::global(), data)
    }

    /// Returns a pointer to the tag in the first byte of this `Data`.
    #[inline(always)]
    pub(crate) unsafe fn tag_ptr(&self) -> *mut u8 {
        mem::transmute::<&Data<'a>, *mut u8>(self)
    }

    /// Returns the tag from the first byte of this `Data`.
    #[inline(always)]
    pub(crate) fn tag(&self) -> u8 {
        unsafe { *self.tag_ptr() }
    }

    /// Returns the type tag from the low 7 bits of the first byte of this `Data`.
    #[inline(always)]
    pub(crate) fn type_tag(&self) -> u8 {
        self.tag() & Value::TYPE_MASK
    }

    pub fn is_empty(&self) -> bool {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, u8>>(&self._1) };
            // Return whether or not the buffer resident is empty.
            buf.is_empty()
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
                // Return whether or not the embedded buffer is empty.
                Some(len) => len == 0,
                None => unreachable!(),
            }
        }
    }

    pub fn len(&self) -> usize {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, u8>>(&self._1) };
            // Return the length of the buffer resident.
            buf.len()
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
                // Return the length of the embedded buffer.
                Some(len) => len as usize,
                None => unreachable!(),
            }
        }
    }

    pub unsafe fn set_len(&mut self, new_len: usize) {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a mutable reference to the buffer lease.
            let buf = mem::transmute::<_, &mut PtrBuf<'a, u8>>(&mut self._1);
            // Set the length of the buffer resident.
            buf.set_len(new_len)
        } else {
            assert!((new_len as u8) < Value::DATA_EMBED_MAX);
            // Make a discriminant tag with the new embedded buffer length.
            let tag = Value::DATA0_TYPE.wrapping_add(new_len as u8);
            // Splice the updated tag into the discriminant.
            let discriminant = self._0.get() & !Value::discriminant(Value::TYPE_MASK) | Value::discriminant(tag);
            // Write the new discriminant.
            ptr::write(&mut self._0, NonZeroU64::new_unchecked(discriminant));
        }
    }

    pub fn cap(&self) -> usize {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, u8>>(&self._1) };
            // Return the capacity of the buffer resident.
            buf.cap()
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
                // Return the capacity of the embedded buffer.
                Some(_) => Value::DATA_EMBED_MAX as usize,
                None => unreachable!(),
            }
        }
    }

    /// Upcasts this `Data` reference to a `Value` reference.
    #[inline]
    pub fn as_value(&self) -> &Value<'a> {
        unsafe { mem::transmute::<&Data<'a>, &Value<'a>>(self) }
    }

    /// Upcasts this `Data` reference to a mutable `Value` reference.
    #[inline]
    pub fn as_mut_value(&mut self) -> &mut Value<'a> {
        unsafe { mem::transmute::<&mut Data<'a>, &mut Value<'a>>(self) }
    }

    /// Upcasts this `Data` to a `Value`.
    #[inline]
    pub fn into_value(self) -> Value<'a> {
        unsafe { mem::transmute::<Data<'a>, Value<'a>>(self) }
    }

    /// Upcasts this `Data` to an `Item`.
    #[inline]
    pub fn into_item(self) -> Item<'a> {
        Item::from_value(self.into_value())
    }

    pub fn as_ptr(&self) -> *const u8 {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, u8>>(&self._1) };
            // Return a pointer to the resident data.
            buf.as_ptr()
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
                // Return a pointer to the embedded buffer.
                Some(_) => (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1),
                None => unreachable!(),
            }
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a mutable reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &mut PtrBuf<'a, u8>>(&mut self._1) };
            // Return a mutable pointer to the resident data.
            buf.as_mut_ptr()
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
                // Return a mutable pointer to the embedded buffer.
                Some(_) => (&mut self._0 as *mut NonZeroU64 as *mut u8).wrapping_add(1),
                None => unreachable!(),
            }
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, u8>>(&self._1) };
            // Return the resident slice.
            buf.as_slice()
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
                Some(len) => {
                    // Get a pointer to the embedded buffer.
                    let embed_ptr = (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1);
                    // Return the embedded slice.
                    unsafe { slice::from_raw_parts(embed_ptr, len as usize) }
                },
                None => unreachable!(),
            }
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &mut PtrBuf<'a, u8>>(&mut self._1) };
            // Return the resident slice.
            buf.as_mut_slice()
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
                Some(len) => {
                    // Get a pointer to the embedded buffer.
                    let embed_ptr = (&mut self._0 as *mut NonZeroU64 as *mut u8).wrapping_add(1);
                    // Return the embedded slice.
                    unsafe { slice::from_raw_parts_mut(embed_ptr, len as usize) }
                },
                None => unreachable!(),
            }
        }
    }

    pub fn try_reserve(&mut self, ext: usize) -> Result<(), HoldError> {
        unsafe {
            let tag = self.tag();
            let type_tag = tag & Value::TYPE_MASK;
            if type_tag == Value::DATA_TYPE {
                // Reconstitute a reference to the buffer lease.
                let buf = mem::transmute::<_, &mut PtrBuf<'a, u8>>(&mut self._1);
                // Reserve capacity in the buffer.
                buf.try_reserve(ext)
            } else {
                match type_tag.checked_sub(Value::DATA0_TYPE) {
                    Some(len) => {
                        if (Value::DATA_EMBED_MAX.wrapping_sub(len) as usize) < ext {
                            // Get a pointer to the embedded buffer.
                            let embed_ptr = (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1);
                            // Reconstruct the placeholder allocation block.
                            let block = Block::from_raw_parts(self._1 as *mut u8, 0);
                            // Get a reference to the hold that allocates this value.
                            let hold = AllocTag::from_ptr(self._1 as *mut u8).holder();
                            // Allocate a new buffer in the hold.
                            let mut buf = PtrBuf::try_hold_cap(hold, len as usize + ext)?;
                            // Copy the embedded slice into the new buffer.
                            ptr::copy_nonoverlapping(embed_ptr, buf.as_mut_ptr(), len as usize);
                            // Set the length of the new buffer.
                            buf.set_len(len as usize);
                            // Modify the discriminant tag with the new record type.
                            let tag = tag & Value::ATTR_FLAG | Value::DATA_TYPE;
                            // Write the new discriminant.
                            ptr::write(&mut self._0, NonZeroU64::new_unchecked(Value::discriminant(tag)));
                            // Write the new data pointer.
                            ptr::write(&mut self._1, PtrBuf::into_raw(buf));
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
            if type_tag == Value::DATA_TYPE {
                // Reconstitute a reference to the buffer lease.
                let buf = mem::transmute::<_, &mut PtrBuf<'a, u8>>(&mut self._1);
                // Reserve capacity in the buffer.
                buf.try_reserve_exact(ext)
            } else {
                match type_tag.checked_sub(Value::DATA0_TYPE) {
                    Some(len) => {
                        if (Value::DATA_EMBED_MAX.wrapping_sub(len) as usize) < ext {
                            // Get a pointer to the embedded buffer.
                            let embed_ptr = (&self._0 as *const NonZeroU64 as *const u8).wrapping_add(1);
                            // Reconstruct the placeholder allocation block.
                            let block = Block::from_raw_parts(self._1 as *mut u8, 0);
                            // Get a reference to the hold that allocates this value.
                            let hold = AllocTag::from_ptr(self._1 as *mut u8).holder();
                            // Allocate a new buffer in the hold.
                            let mut buf = PtrBuf::try_hold_cap(hold, len as usize + ext)?;
                            // Copy the embedded slice into the new buffer.
                            ptr::copy_nonoverlapping(embed_ptr, buf.as_mut_ptr(), len as usize);
                            // Set the length of the new buffer.
                            buf.set_len(len as usize);
                            // Modify the discriminant tag with the new record type.
                            let tag = tag & Value::ATTR_FLAG | Value::DATA_TYPE;
                            // Write the new discriminant.
                            ptr::write(&mut self._0, NonZeroU64::new_unchecked(Value::discriminant(tag)));
                            // Write the new data pointer.
                            ptr::write(&mut self._1, PtrBuf::into_raw(buf));
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
            if type_tag == Value::DATA_TYPE {
                // Reconstitute a reference to the buffer lease.
                let buf = mem::transmute::<_, &mut PtrBuf<'a, u8>>(&mut self._1);
                // Reserve capacity in the buffer.
                buf.try_reserve_in_place(ext)
            } else {
                match type_tag.checked_sub(Value::DATA0_TYPE) {
                    Some(len) => {
                        if (Value::DATA_EMBED_MAX.wrapping_sub(len) as usize) >= ext {
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
            if type_tag == Value::DATA_TYPE {
                // Reconstitute a reference to the buffer lease.
                let buf = mem::transmute::<_, &mut PtrBuf<'a, u8>>(&mut self._1);
                // Reserve capacity in the buffer.
                buf.try_reserve_in_place_exact(ext)
            } else {
                match type_tag.checked_sub(Value::DATA0_TYPE) {
                    Some(len) => {
                        if (Value::DATA_EMBED_MAX.wrapping_sub(len) as usize) >= ext {
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

    pub fn try_push(&mut self, value: u8) -> Result<(), HoldError> {
        unsafe {
            self.try_reserve(1)?;
            let len = self.len();
            let data = self.as_mut_ptr().wrapping_add(len);
            ptr::write(data, value);
            self.set_len(len.wrapping_add(1));
            Ok(())
        }
    }

    pub fn push(&mut self, value: u8) {
        self.try_push(value).unwrap();
    }

    pub fn try_insert(&mut self, index: usize, value: u8) -> Result<(), HoldError> {
        unsafe {
            let len = self.len();
            assert!(index <= len);
            self.try_reserve(1)?;
            let data = self.as_mut_ptr().wrapping_add(len);
            ptr::copy(data, data.wrapping_add(1), len.wrapping_sub(index));
            ptr::write(data, value);
            self.set_len(len.wrapping_add(1));
            Ok(())
        }
    }

    pub fn insert(&mut self, index: usize, value: u8) {
        self.try_insert(index, value).unwrap();
    }

    pub fn try_insert_slice(&mut self, index: usize, slice: &[u8]) -> Result<(), HoldError> {
        unsafe {
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
    }

    pub fn insert_slice(&mut self, index: usize, slice: &[u8]) {
        self.try_insert_slice(index, slice).unwrap();
    }

    pub fn pop(&mut self) -> Option<u8> {
        unsafe {
            let len = self.len();
            if len != 0 {
                let len = len.wrapping_sub(1);
                self.set_len(len);
                let data = self.as_ptr().wrapping_add(len);
                Some(ptr::read(data))
            } else {
                None
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> u8 {
        unsafe {
            let len = self.len();
            assert!(index <= len);
            let data = self.as_mut_ptr().wrapping_add(index);
            let value = ptr::read(data);
            ptr::copy(data.wrapping_add(1), data, len.wrapping_sub(index).wrapping_add(1));
            self.set_len(len.wrapping_sub(1));
            value
        }
    }

    pub fn truncate(&mut self, new_len: usize) {
        unsafe {
            let old_len = self.len();
            if old_len > new_len {
                self.set_len(new_len);
            }
        }
    }

    pub fn clear(&mut self) {
        self.truncate(0);
    }

    pub(crate) unsafe fn dealloc(&mut self) {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconsitute the Hold-allocated buffer.
            let buf = PtrBuf::<'a, u8, ()>::from_raw(self._1);
            // And drop it.
            mem::drop(buf);
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
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

impl<'a> Holder<'a> for Data<'a> {
    #[inline]
    fn holder(&self) -> &'a dyn Hold<'a> {
        let type_tag = self.type_tag();
        if type_tag == Value::DATA_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, u8>>(&self._1) };
            // Return the buffer resident holder.
            buf.holder()
        } else {
            match type_tag.checked_sub(Value::DATA0_TYPE) {
                // Return the embedded buffer holder.
                Some(_) => AllocTag::from_ptr(self._1).holder(),
                None => unreachable!(),
            }
        }
    }
}

impl<'a> Deref for Data<'a> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> DerefMut for Data<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<'a> AsRef<Value<'a>> for Data<'a> {
    #[inline]
    fn as_ref(&self) -> &Value<'a> {
        self.as_value()
    }
}

impl<'a> AsRef<[u8]> for Data<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> AsMut<Value<'a>> for Data<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Value<'a> {
        self.as_mut_value()
    }
}

impl<'a> AsMut<[u8]> for Data<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<'a, Idx: SliceIndex<[u8]>> Index<Idx> for Data<'a> {
    type Output = Idx::Output;

    #[inline]
    fn index(&self, index: Idx) -> &Idx::Output {
        self.as_slice().index(index)
    }
}

impl<'a, Idx: SliceIndex<[u8]>> IndexMut<Idx> for Data<'a> {
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Idx::Output {
        self.as_mut_slice().index_mut(index)
    }
}

impl<'a> PartialEq for Data<'a> {
    #[inline]
    fn eq(&self, that: &Data<'a>) -> bool {
        self.as_slice().eq(that.as_slice())
    }

    #[inline]
    fn ne(&self, that: &Data<'a>) -> bool {
        self.as_slice().ne(that.as_slice())
    }
}

impl<'a> cmp::PartialOrd<Data<'a>> for Data<'a> {
    #[inline]
    fn partial_cmp(&self, that: &Data<'a>) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(that.as_slice())
    }

    #[inline]
    fn lt(&self, that: &Data<'a>) -> bool {
        self.as_slice().lt(that.as_slice())
    }

    #[inline]
    fn le(&self, that: &Data<'a>) -> bool {
        self.as_slice().le(that.as_slice())
    }

    #[inline]
    fn ge(&self, that: &Data<'a>) -> bool {
        self.as_slice().ge(that.as_slice())
    }

    #[inline]
    fn gt(&self, that: &Data<'a>) -> bool {
        self.as_slice().gt(that.as_slice())
    }
}

impl<'a> hash::Hash for Data<'a> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_slice().hash(hasher);
    }
}

impl<'a> fmt::Debug for Data<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!(); // TODO
    }
}

impl<'a> Clone for Data<'a> {
    fn clone(&self) -> Data<'a> {
        Data::from_slice(self.as_slice())
    }
}

impl<'a> TryClone for Data<'a> {
    fn try_clone(&self) -> Result<Data<'a>, HoldError> {
        self.try_clone_into_hold(self.holder())
    }
}

impl<'a, 'b> CloneIntoHold<'a, Data<'a>> for Data<'b> {
    fn try_clone_into_hold(&self, hold: &Hold<'a>) -> Result<Data<'a>, HoldError> {
        Data::try_hold_slice(hold, self.as_slice())
    }
}

impl<'a, 'b> Stow<'b, Data<'b>> for Data<'a> {
    unsafe fn stow(src: *mut Data<'a>, dst: *mut Data<'b>, hold: &Hold<'b>) -> Result<(), HoldError> {
        let len = (*src).len();
        if len <= Value::DATA_EMBED_MAX as usize {
            let block = hold.alloc(Layout::empty())?;
            ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(Value::DATA0_TYPE + len as u8)));
            ptr::write(&mut (*dst)._1, block.into_raw());
            let embed_ptr = (&mut (*dst)._0 as *mut NonZeroU64 as *mut u8).wrapping_add(1);
            ptr::copy_nonoverlapping((*src).as_ptr(), embed_ptr, len);
        } else {
            let buf = PtrBuf::try_hold_copy(hold, (*src).as_slice())?;
            ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(Value::DATA_TYPE)));
            ptr::write(&mut (*dst)._1, PtrBuf::into_raw(buf));
        }
        Ok(())
    }

    unsafe fn unstow(_src: *mut Data<'a>, _dst: *mut Data<'b>) {
        unimplemented!();
    }
}

impl<'a> Drop for Data<'a> {
    fn drop(&mut self) {
        unsafe { self.dealloc(); }
    }
}
