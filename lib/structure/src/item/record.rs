use core::cmp;
use core::fmt;
use core::hash;
use core::marker::PhantomData;
use core::mem;
use core::num::NonZeroU64;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr;
use core::slice::{self, SliceIndex};
use swim_mem::block::{Block, Layout, ZSP};
use swim_mem::alloc::{Hold, Holder, HoldError, AllocTag, Stow, TryClone, CloneIntoHold};
use swim_mem::lease::PtrBuf;
use crate::item::{Item, Value};

/// `Value` variant representing a collection of `Item`s.
#[derive(Eq, Ord)]
#[repr(C)]
pub struct Record<'a> {
    /// Discriminant with a type between `RECORD_TYPE_MIN` and `RECORD_TYPE_MAX`
    /// at the lowest byte address.
    ///
    /// ```text
    /// 0        1        2        3        4        5        6        7        8
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// |  type  |      0 |      0 |      0 |      0 |      0 |      0 |      0 |
    /// +--------+--------+--------+--------+--------+--------+--------+--------+
    /// ```
    _0: NonZeroU64,
    /// Raw pointer to either a PtrBuf<'a, Item<'a>>, or an empty placeholder allocation.
    _1: *mut Item<'a>,
    /// Variant over allocation lifetime.
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Record<'a> {
    pub fn try_hold_slice<'b>(hold: &dyn Hold<'a>, data: &[Item<'b>]) -> Result<Record<'a>, HoldError> {
        unsafe {
            let len = data.len();
            if len == 0 {
                let block = hold.alloc(Layout::empty())?;
                Ok(Record {
                    _0: NonZeroU64::new_unchecked(Value::discriminant(Value::RECORD0_TYPE)),
                    _1: block.into_raw() as *mut Item<'a>,
                    lifetime: PhantomData,
                })
            } else {
                let buf = PtrBuf::try_hold_clone(hold, data)?;
                Ok(Record {
                    _0: NonZeroU64::new_unchecked(Value::discriminant(Value::RECORD_TYPE)),
                    _1: PtrBuf::into_raw(buf),
                    lifetime: PhantomData,
                })
            }
        }
    }

    pub fn hold_slice<'b>(hold: &dyn Hold<'a>, data: &[Item<'b>]) -> Record<'a> {
        Record::try_hold_slice(hold, data).unwrap()
    }

    pub fn from_slice<'b>(data: &[Item<'b>]) -> Record<'a> {
        Record::hold_slice(Hold::global(), data)
    }

    /// Returns a pointer to the tag in the first byte of this `Record`.
    #[inline(always)]
    pub(crate) unsafe fn tag_ptr(&self) -> *mut u8 {
        mem::transmute::<&Record<'a>, *mut u8>(self)
    }

    /// Returns the tag from the first byte of this `Record`.
    #[inline(always)]
    pub(crate) fn tag(&self) -> u8 {
        unsafe { *self.tag_ptr() }
    }

    /// Returns the type tag from the low 7 bits of the first byte of this `Record`.
    #[inline(always)]
    pub(crate) fn type_tag(&self) -> u8 {
        self.tag() & Value::TYPE_MASK
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        let type_tag = self.type_tag();
        if type_tag == Value::RECORD_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, Item<'a>>>(&self._1) };
            // Return whether or not the buffer resident is empty.
            buf.is_empty()
        } else if type_tag == Value::RECORD0_TYPE {
            true
        } else {
            unreachable!();
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        let type_tag = self.type_tag();
        if type_tag == Value::RECORD_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, Item<'a>>>(&self._1) };
            // Return the length of the buffer resident.
            buf.len()
        } else if type_tag == Value::RECORD0_TYPE {
            0
        } else {
            unreachable!();
        }
    }

    #[inline]
    pub fn cap(&self) -> usize {
        let type_tag = self.type_tag();
        if type_tag == Value::RECORD_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, Item<'a>>>(&self._1) };
            // Return the capacity of the buffer resident.
            buf.cap()
        } else if type_tag == Value::RECORD0_TYPE {
            0
        } else {
            unreachable!();
        }
    }

    /// Upcasts this `Record` reference to a `Value` reference.
    #[inline]
    pub fn as_value(&self) -> &Value<'a> {
        unsafe { mem::transmute::<&Record<'a>, &Value<'a>>(self) }
    }

    /// Upcasts this `Record` reference to a mutable `Value` reference.
    #[inline]
    pub fn as_mut_value(&mut self) -> &mut Value<'a> {
        unsafe { mem::transmute::<&mut Record<'a>, &mut Value<'a>>(self) }
    }

    /// Upcasts this `Record` to a `Value`.
    #[inline]
    pub fn into_value(self) -> Value<'a> {
        unsafe { mem::transmute::<Record<'a>, Value<'a>>(self) }
    }

    /// Upcasts this `Record` to an `Item`.
    #[inline]
    pub fn into_item(self) -> Item<'a> {
        Item::from_value(self.into_value())
    }

    pub fn as_slice(&self) -> &[Item<'a>] {
        let type_tag = self.type_tag();
        if type_tag == Value::RECORD_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, Item<'a>>>(&self._1) };
            // Return the resident slice.
            buf.as_slice()
        } else if type_tag == Value::RECORD0_TYPE {
            // Return an empty slice.
            unsafe { slice::from_raw_parts(ZSP as *const Item<'a>, 0) }
        } else {
            unreachable!();
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [Item<'a>] {
        let type_tag = self.type_tag();
        if type_tag == Value::RECORD_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1) };
            // Return the resident slice.
            buf.as_mut_slice()
        } else if type_tag == Value::RECORD0_TYPE {
            // Return an empty slice.
            unsafe { slice::from_raw_parts_mut(ZSP as *mut Item<'a>, 0) }
        } else {
            unreachable!();
        }
    }

    pub fn try_reserve(&mut self, ext: usize) -> Result<(), HoldError> {
        unsafe {
            let tag = self.tag();
            let type_tag = tag & Value::TYPE_MASK;
            if type_tag == Value::RECORD_TYPE {
                // Reconstitute a reference to the buffer lease.
                let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
                // Reserve capacity in the buffer.
                buf.try_reserve(ext)
            } else if type_tag == Value::RECORD0_TYPE {
                if ext != 0 {
                    // Reconstruct the placeholder allocation block.
                    let block = Block::from_raw_parts(self._1 as *mut u8, 0);
                    // Get a reference to the hold that allocates this value.
                    let hold = AllocTag::from_ptr(self._1 as *mut u8).holder();
                    // Allocate a new buffer in the hold.
                    let buf = PtrBuf::try_hold_cap(hold, ext)?;
                    // Modify the discriminant tag with the new record type.
                    let tag = tag & Value::ATTR_FLAG | Value::RECORD_TYPE;
                    // Write the new discriminant.
                    ptr::write(&mut self._0, NonZeroU64::new_unchecked(Value::discriminant(tag)));
                    // Write the new data pointer.
                    ptr::write(&mut self._1, PtrBuf::into_raw(buf));
                    // Deallocate the placeholder block.
                    hold.dealloc(block);
                }
                Ok(())
            } else {
                unreachable!();
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
            if type_tag == Value::RECORD_TYPE {
                // Reconstitute a reference to the buffer lease.
                let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
                // Reserve capacity in the buffer.
                buf.try_reserve_exact(ext)
            } else if type_tag == Value::RECORD0_TYPE {
                if ext != 0 {
                    // Reconstruct the placeholder allocation block.
                    let block = Block::from_raw_parts(self._1 as *mut u8, 0);
                    // Get a reference to the hold that allocates this value.
                    let hold = AllocTag::from_ptr(self._1 as *mut u8).holder();
                    // Allocate a new buffer in the hold.
                    let buf = PtrBuf::try_hold_cap(hold, ext)?;
                    // Modify the discriminant tag with the new record type.
                    let tag = tag & Value::ATTR_FLAG | Value::RECORD_TYPE;
                    // Write the new discriminant.
                    ptr::write(&mut self._0, NonZeroU64::new_unchecked(Value::discriminant(tag)));
                    // Write the new data pointer.
                    ptr::write(&mut self._1, PtrBuf::into_raw(buf));
                    // Deallocate the placeholder block.
                    hold.dealloc(block);
                }
                Ok(())
            } else {
                unreachable!();
            }
        }
    }

    pub fn reserve_exact(&mut self, ext: usize) {
        self.try_reserve_exact(ext).unwrap();
    }

    pub fn try_reserve_in_place(&mut self, ext: usize) -> Result<(), HoldError> {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag == Value::RECORD_TYPE {
                // Reconstitute a reference to the buffer lease.
                let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
                // Reserve capacity in the buffer.
                buf.try_reserve_in_place(ext)
            } else if type_tag == Value::RECORD0_TYPE {
                if ext == 0 {
                    Ok(())
                } else {
                    Err(HoldError::Unsupported("resize from empty"))
                }
            } else {
                unreachable!();
            }
        }
    }

    pub fn try_reserve_in_place_exact(&mut self, ext: usize) -> Result<(), HoldError> {
        unsafe {
            let type_tag = self.type_tag();
            if type_tag == Value::RECORD_TYPE {
                // Reconstitute a reference to the buffer lease.
                let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
                // Reserve capacity in the buffer.
                buf.try_reserve_in_place_exact(ext)
            } else if type_tag == Value::RECORD0_TYPE {
                if ext == 0 {
                    Ok(())
                } else {
                    Err(HoldError::Unsupported("resize from empty"))
                }
            } else {
                unreachable!();
            }
        }
    }

    pub fn try_push(&mut self, item: Item<'a>) -> Result<(), HoldError> {
        unsafe {
            self.try_reserve(1)?;
            let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
            let len = buf.len();
            let data = buf.as_mut_ptr().wrapping_add(len);
            ptr::write(data, item);
            buf.set_len(len.wrapping_add(1));
            Ok(())
        }
    }

    pub fn push(&mut self, item: Item<'a>) {
        self.try_push(item).unwrap();
    }

    pub fn try_insert(&mut self, index: usize, item: Item<'a>) -> Result<(), HoldError> {
        unsafe {
            let len = self.len();
            assert!(index <= len);
            self.try_reserve(1)?;
            let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
            let data = buf.as_mut_ptr().wrapping_add(len);
            ptr::copy(data, data.wrapping_add(1), len.wrapping_sub(index));
            ptr::write(data, item);
            buf.set_len(len.wrapping_add(1));
            Ok(())
        }
    }

    pub fn insert(&mut self, index: usize, item: Item<'a>) {
        self.try_insert(index, item).unwrap();
    }

    pub fn pop(&mut self) -> Option<Item<'a>> {
        unsafe {
            let len = self.len();
            if len != 0 {
                let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
                let len = len.wrapping_sub(1);
                buf.set_len(len);
                let data = buf.as_ptr().wrapping_add(len);
                Some(ptr::read(data))
            } else {
                None
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> Item<'a> {
        unsafe {
            let len = self.len();
            assert!(index <= len);
            let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
            let data = buf.as_mut_ptr().wrapping_add(index);
            let item = ptr::read(data);
            ptr::copy(data.wrapping_add(1), data, len.wrapping_sub(index).wrapping_add(1));
            buf.set_len(len.wrapping_sub(1));
            item
        }
    }

    pub fn truncate(&mut self, new_len: usize) {
        unsafe {
            let old_len = self.len();
            if old_len > new_len {
                let buf = mem::transmute::<_, &mut PtrBuf<'a, Item<'a>>>(&mut self._1);
                let tail = buf.as_mut_ptr().wrapping_add(new_len);
                ptr::drop_in_place(slice::from_raw_parts_mut(tail, old_len.wrapping_sub(new_len)));
                buf.set_len(new_len);
            }
        }
    }

    pub fn clear(&mut self) {
        self.truncate(0);
    }

    pub(crate) unsafe fn dealloc(&mut self) {
        let type_tag = self.type_tag();
        if type_tag == Value::RECORD_TYPE {
            // Reconsitute the Hold-allocated buffer.
            let buf = PtrBuf::<'a, Item<'a>, ()>::from_raw(self._1);
            // And drop it.
            mem::drop(buf);
        } else if type_tag == Value::RECORD0_TYPE {
            // Reconstruct the empty placeholder allocation block.
            let block = Block::from_raw_parts(self._1 as *mut u8, 0);
            // Get a pointer to the AllocTag of the block,
            // and use it to deallocate the empty placeholder block.
            AllocTag::from_ptr(self._1 as *mut u8).dealloc(block);
        } else {
            unreachable!();
        }
    }
}

impl<'a> Holder<'a> for Record<'a> {
    #[inline]
    fn holder(&self) -> &'a dyn Hold<'a> {
        let type_tag = self.type_tag();
        if type_tag == Value::RECORD_TYPE {
            // Reconstitute a reference to the buffer lease.
            let buf = unsafe { mem::transmute::<_, &PtrBuf<'a, Item<'a>>>(&self._1) };
            // Return the buffer resident holder.
            buf.holder()
        } else if type_tag == Value::RECORD0_TYPE {
            // Return the empty placeholder allocation holder.
            AllocTag::from_ptr(self._1 as *mut u8).holder()
        } else {
            unreachable!();
        }
    }
}

impl<'a> Deref for Record<'a> {
    type Target = [Item<'a>];

    #[inline]
    fn deref(&self) -> &[Item<'a>] {
        self.as_slice()
    }
}

impl<'a> DerefMut for Record<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Item<'a>] {
        self.as_mut_slice()
    }
}

impl<'a> AsRef<Value<'a>> for Record<'a> {
    #[inline]
    fn as_ref(&self) -> &Value<'a> {
        self.as_value()
    }
}

impl<'a> AsRef<[Item<'a>]> for Record<'a> {
    #[inline]
    fn as_ref(&self) -> &[Item<'a>] {
        self.as_slice()
    }
}

impl<'a> AsMut<Value<'a>> for Record<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Value<'a> {
        self.as_mut_value()
    }
}

impl<'a> AsMut<[Item<'a>]> for Record<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut [Item<'a>] {
        self.as_mut_slice()
    }
}

impl<'a, Idx: SliceIndex<[Item<'a>]> + 'a> Index<Idx> for Record<'a> {
    type Output = Idx::Output;

    #[inline]
    fn index(&self, index: Idx) -> &Idx::Output {
        self.as_slice().index(index)
    }
}

impl<'a, Idx: SliceIndex<[Item<'a>]> + 'a> IndexMut<Idx> for Record<'a> {
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Idx::Output {
        self.as_mut_slice().index_mut(index)
    }
}

impl<'a> PartialEq for Record<'a> {
    #[inline]
    fn eq(&self, that: &Record<'a>) -> bool {
        self.as_slice().eq(that.as_slice())
    }

    #[inline]
    fn ne(&self, that: &Record<'a>) -> bool {
        self.as_slice().ne(that.as_slice())
    }
}

impl<'a> cmp::PartialOrd<Record<'a>> for Record<'a> {
    #[inline]
    fn partial_cmp(&self, that: &Record<'a>) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(that.as_slice())
    }

    #[inline]
    fn lt(&self, that: &Record<'a>) -> bool {
        self.as_slice().lt(that.as_slice())
    }

    #[inline]
    fn le(&self, that: &Record<'a>) -> bool {
        self.as_slice().le(that.as_slice())
    }

    #[inline]
    fn ge(&self, that: &Record<'a>) -> bool {
        self.as_slice().ge(that.as_slice())
    }

    #[inline]
    fn gt(&self, that: &Record<'a>) -> bool {
        self.as_slice().gt(that.as_slice())
    }
}

impl<'a> hash::Hash for Record<'a> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_slice().hash(hasher);
    }
}

impl<'a> fmt::Debug for Record<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!(); // TODO
    }
}

impl<'a> Clone for Record<'a> {
    fn clone(&self) -> Record<'a> {
        Record::from_slice(self.as_slice())
    }
}

impl<'a> TryClone for Record<'a> {
    fn try_clone(&self) -> Result<Record<'a>, HoldError> {
        self.try_clone_into_hold(self.holder())
    }
}

impl<'a, 'b> CloneIntoHold<'a, Record<'a>> for Record<'b> {
    fn try_clone_into_hold(&self, hold: &Hold<'a>) -> Result<Record<'a>, HoldError> {
        Record::try_hold_slice(hold, self.as_slice())
    }
}

impl<'a, 'b> Stow<'b, Record<'b>> for Record<'a> {
    unsafe fn stow(src: *mut Record<'a>, dst: *mut Record<'b>, hold: &Hold<'b>) -> Result<(), HoldError> {
        let len = (*src).len();
        if len == 0 {
            let block = hold.alloc(Layout::empty())?;
            ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(Value::RECORD0_TYPE)));
            ptr::write(&mut (*dst)._1, block.into_raw() as *mut Item<'b>);
        } else {
            let buf = PtrBuf::try_hold_clone(hold, (*src).as_slice())?;
            ptr::write(&mut (*dst)._0, NonZeroU64::new_unchecked(Value::discriminant(Value::RECORD_TYPE)));
            ptr::write(&mut (*dst)._1, PtrBuf::into_raw(buf));
        }
        Ok(())
    }

    unsafe fn unstow(_src: *mut Record<'a>, _dst: *mut Record<'b>) {
        unimplemented!();
    }
}

impl<'a> Drop for Record<'a> {
    fn drop(&mut self) {
        unsafe { self.dealloc(); }
    }
}
