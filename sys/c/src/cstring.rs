use core::cmp::Ordering;
use core::fmt::{self, Debug, Formatter, Write};
use core::hash::{Hash, Hasher};
use core::intrinsics::assume;
use core::marker::PhantomData;
use core::mem;
use core::ops::Deref;
use core::ptr;
use core::slice;
use core::str::{self, Utf8Error};
use swim_mem::block::{Layout, LayoutError};
use swim_mem::alloc::{Hold, HoldError, TryClone};
use swim_mem::lease::{Lease, DynamicLease, Raw, Ptr, Mut, Ref, Hard, Soft};
use swim_mem::resident::{Resident, ResidentFromCopy, ResidentFromCopyUnchecked,
                         ResidentFromEmpty, ResidentWithCapacity, ResidentDeref,
                         ResidentDerefMut, ResidentAsRef, ResidentAdd,
                         ResidentAddAssign, ResidentPartialEq, ResidentEq,
                         ResidentPartialOrd, ResidentOrd, ResidentHash,
                         ResidentDebug, ResidentClone, ResidentStow,
                         BufHeader, BufLease};
use swim_c_sys::{cchar, void};
use swim_c_sys::string;
use crate::cstr::CStr;

pub type RawCString<'a, M = ()> = Raw<'a, CString<M>>;
pub type PtrCString<'a, M = ()> = Ptr<'a, CString<M>>;
pub type MutCString<'a, M = ()> = Mut<'a, CString<M>>;
pub type RefCString<'a, M = ()> = Ref<'a, CString<M>>;
pub type HardCString<'a, M = ()> = Hard<'a, CString<M>>;
pub type SoftCString<'a, M = ()> = Soft<'a, CString<M>>;

/// A nul-terminated array of C `char`s allocated in a `Hold`.
pub struct CString<M = ()> {
    /// Variant over BufHeader<M>, with drop check.
    meta_marker: PhantomData<BufHeader<M>>,
}

pub struct CStringLease<L: Lease<Data=u8, Meta=BufHeader<M>>, M = ()> {
    /// Memory `Lease` in which the `CString` resides.
    lease: L,
}

unsafe impl<M: Send> Send for CString<M> {
}

unsafe impl<M: Sync> Sync for CString<M> {
}

impl<M> CString<M> {
    #[inline]
    fn header(lease: &impl Lease<Data=u8, Meta=BufHeader<M>>) -> &BufHeader<M> {
        unsafe { &*lease.meta() }
    }

    #[inline]
    fn header_mut(lease: &mut impl Lease<Data=u8, Meta=BufHeader<M>>) -> &mut BufHeader<M> {
        unsafe { &mut *lease.meta() }
    }

    #[inline]
    fn len(lease: &impl Lease<Data=u8, Meta=BufHeader<M>>) -> usize {
        let len = CString::header(lease).len;
        if len != 0 { len.wrapping_sub(1) } else { len }
    }

    #[inline]
    fn as_slice(lease: &impl Lease<Data=u8, Meta=BufHeader<M>>) -> &[u8] {
        unsafe {
            let data = lease.data();
            assume(!data.is_null());
            slice::from_raw_parts(data, CString::header(lease).len)
        }
    }

    #[inline]
    fn as_str(lease: &impl Lease<Data=u8, Meta=BufHeader<M>>) -> Result<&str, Utf8Error> {
        unsafe {
            let data = lease.data();
            assume(!data.is_null());
            str::from_utf8(slice::from_raw_parts_mut(data, CString::len(lease)))
        }
    }

    #[inline]
    unsafe fn as_str_unchecked(lease: &impl Lease<Data=u8, Meta=BufHeader<M>>) -> &str {
        let data = lease.data();
        assume(!data.is_null());
        str::from_utf8_unchecked(slice::from_raw_parts_mut(data, CString::len(lease)))
    }

    #[inline]
    fn as_cstr(lease: &impl Lease<Data=u8, Meta=BufHeader<M>>) -> &CStr {
        CStr::from_ptr(lease.data())
    }
}

impl<M> Resident for CString<M> {
    type Data = u8;

    type Meta = BufHeader<M>;

    #[inline]
    unsafe fn resident_size(_data: *mut u8, meta: *mut BufHeader<M>) -> usize {
        (*meta).cap
    }

    #[inline]
    unsafe fn resident_drop(_data: *mut u8, _meta: *mut BufHeader<M>) {
        // nop
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentFromCopy<L, [u8], M> for CString<M> {
    #[inline]
    fn new_resident_layout(data: &[u8], _meta: &M) -> Layout {
        let len = data.len().checked_add(1).unwrap();
        unsafe { Layout::for_array_unchecked::<u8>(len) }
    }

    #[inline]
    fn new_resident_ptr(raw: *mut u8, _data: &[u8], _meta: &M) -> *mut u8 {
        raw
    }

    #[inline]
    fn new_resident(lease: &mut L, data: &[u8], meta: M) {
        unsafe {
            if string::memchr(data.as_ptr() as *mut void, 0, data.len()).is_null() {
                let len = data.len();
                let cap = len.wrapping_add(1);
                ptr::write(lease.meta(), BufHeader {
                    len: cap,
                    cap: cap,
                    meta: meta,
                });
                ptr::copy_nonoverlapping(data.as_ptr(), lease.data(), len);
                ptr::write(lease.data().wrapping_add(len), 0);
            } else {
                panic!("nul byte");
            }
        }
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentFromCopyUnchecked<L, [u8], M> for CString<M> {
    #[inline]
    fn new_resident_layout(data: &[u8], _meta: &M) -> Layout {
        Layout::for_value(data)
    }

    #[inline]
    fn new_resident_ptr(raw: *mut u8, _data: &[u8], _meta: &M) -> *mut u8 {
        raw
    }

    #[inline]
    fn new_resident(lease: &mut L, data: &[u8], meta: M) {
        unsafe {
            let len = data.len();
            ptr::write(lease.meta(), BufHeader {
                len: len,
                cap: len,
                meta: meta,
            });
            ptr::copy_nonoverlapping(data.as_ptr(), lease.data(), len);
        }
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentFromCopy<L, str, M> for CString<M> {
    #[inline]
    fn new_resident_layout(data: &str, meta: &M) -> Layout {
        <CString<M> as ResidentFromCopy<L, [u8], M>>::new_resident_layout(data.as_bytes(), meta)
    }

    #[inline]
    fn new_resident_ptr(raw: *mut u8, data: &str, meta: &M) -> *mut u8 {
        <CString<M> as ResidentFromCopy<L, [u8], M>>::new_resident_ptr(raw, data.as_bytes(), meta)
    }

    #[inline]
    fn new_resident(lease: &mut L, data: &str, meta: M) {
        <CString<M> as ResidentFromCopy<L, [u8], M>>::new_resident(lease, data.as_bytes(), meta)
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentFromCopyUnchecked<L, CStr, M> for CString<M> {
    #[inline]
    fn new_resident_layout(data: &CStr, meta: &M) -> Layout {
        let slice = unsafe { CStr::to_bytes(data) };
        <CString<M> as ResidentFromCopyUnchecked<L, [u8], M>>::new_resident_layout(slice, meta)
    }

    #[inline]
    fn new_resident_ptr(raw: *mut u8, data: &CStr, meta: &M) -> *mut u8 {
        let slice = unsafe { CStr::to_bytes(data) };
        <CString<M> as ResidentFromCopyUnchecked<L, [u8], M>>::new_resident_ptr(raw, slice, meta)
    }

    #[inline]
    fn new_resident(lease: &mut L, data: &CStr, meta: M) {
        let slice = unsafe { CStr::to_bytes(data) };
        <CString<M> as ResidentFromCopyUnchecked<L, [u8], M>>::new_resident(lease, slice, meta)
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentFromEmpty<L, M> for CString<M> {
    #[inline]
    fn new_resident_layout(_meta: &M) -> Layout {
        Layout::empty()
    }

    #[inline]
    fn new_resident_ptr(raw: *mut u8, _meta: &M) -> *mut u8 {
        raw
    }

    #[inline]
    fn new_resident(lease: &mut L, meta: M) {
        unsafe {
            ptr::write(lease.meta(), BufHeader {
                len: 0,
                cap: 0,
                meta: meta,
            });
        }
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentWithCapacity<L, M> for CString<M> {
    #[inline]
    fn new_resident_layout(cap: usize, _meta: &M) -> Result<Layout, LayoutError> {
        Layout::for_array::<u8>(cap)
    }

    #[inline]
    fn new_resident_ptr(raw: *mut u8, _cap: usize, _meta: &M) -> *mut u8 {
        raw
    }

    #[inline]
    fn new_resident(lease: &mut L, cap: usize, meta: M) {
        unsafe {
            ptr::write(lease.meta(), BufHeader {
                len: 0,
                cap: cap,
                meta: meta,
            });
        }
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentDeref<L> for CString<M> {
    type Target = CStringLease<L, M>;

    #[inline]
    fn resident_deref(lease: &L) -> &CStringLease<L, M> {
        unsafe { mem::transmute::<&L, &CStringLease<L, M>>(lease) }
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentDerefMut<L> for CString<M> {
    #[inline]
    fn resident_deref_mut(lease: &mut L) -> &mut CStringLease<L, M> {
        unsafe { mem::transmute::<&mut L, &mut CStringLease<L, M>>(lease) }
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentAsRef<L, CStr> for CString<M> {
    #[inline]
    fn resident_as_ref(lease: &L) -> &CStr {
        CString::as_cstr(lease)
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentAsRef<L, [u8]> for CString<M> {
    #[inline]
    fn resident_as_ref(lease: &L) -> &[u8] {
        CString::as_slice(lease)
    }
}

impl<'a, 'b, L: DynamicLease<'a, Data=u8, Meta=BufHeader<M>>, M> ResidentAdd<L, &'b str> for CString<M> {
    type Output = L;

    #[inline]
    fn resident_add(mut lease: L, rhs: &'b str) -> L {
        CString::resident_deref_mut(&mut lease).push_str(rhs);
        lease
    }
}

impl<'a, 'b, L: DynamicLease<'a, Data=u8, Meta=BufHeader<M>>, M> ResidentAddAssign<L, &'b str> for CString<M> {
    #[inline]
    fn resident_add_assign(lease: &mut L, rhs: &'b str) {
        CString::resident_deref_mut(lease).push_str(rhs);
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentPartialEq<L> for CString<M> {
    #[inline]
    fn resident_eq(lease: &L, other: &L) -> bool {
        CString::as_slice(lease).eq(CString::as_slice(other))
    }

    #[inline]
    fn resident_ne(lease: &L, other: &L) -> bool {
        CString::as_slice(lease).ne(CString::as_slice(other))
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentEq<L> for CString<M> {
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentPartialOrd<L> for CString<M> {
    #[inline]
    fn resident_partial_cmp(lease: &L, other: &L) -> Option<Ordering> {
        CString::as_slice(lease).partial_cmp(CString::as_slice(other))
    }

    #[inline]
    fn resident_lt(lease: &L, other: &L) -> bool {
        CString::as_slice(lease).lt(CString::as_slice(other))
    }

    #[inline]
    fn resident_le(lease: &L, other: &L) -> bool {
        CString::as_slice(lease).le(CString::as_slice(other))
    }

    #[inline]
    fn resident_ge(lease: &L, other: &L) -> bool {
        CString::as_slice(lease).ge(CString::as_slice(other))
    }

    #[inline]
    fn resident_gt(lease: &L, other: &L) -> bool {
        CString::as_slice(lease).gt(CString::as_slice(other))
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentOrd<L> for CString<M> {
    #[inline]
    fn resident_cmp(lease: &L, other: &L) -> Ordering {
        CString::as_slice(lease).cmp(CString::as_slice(other))
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentHash<L> for CString<M> {
    #[inline]
    fn resident_hash<H: Hasher>(lease: &L, state: &mut H) {
        CString::as_slice(lease).hash(state)
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> ResidentDebug<L> for CString<M> {
    #[inline]
    fn resident_fmt(lease: &L, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(CString::as_str(lease).unwrap(), f)
    }
}

impl<L1, L2, M> ResidentClone<L1, L2> for CString<M>
    where L1: Lease<Data=u8, Meta=BufHeader<M>>,
          L2: Lease<Data=u8, Meta=BufHeader<M>>,
          M: TryClone,
{
    #[inline]
    fn new_resident_layout(lease: &L1) -> Layout {
        unsafe { Layout::for_array_unchecked::<u8>(CString::header(lease).len) }
    }

    #[inline]
    fn resident_clone(src: &L1, dst: &mut L2) -> Result<(), HoldError> {
        unsafe {
            let src_meta = src.meta();
            let dst_meta = dst.meta();
            ptr::write(dst_meta, (*src_meta).try_clone()?);
            let len = (*dst_meta).len;
            (*dst_meta).cap = len;
            ptr::copy_nonoverlapping(src.data(), dst.data(), len);
            Ok(())
        }
    }
}

impl<'b, L1, L2, M> ResidentStow<'b, L1, L2> for CString<M>
    where L1: Lease<Data=u8, Meta=BufHeader<M>>,
          L2: Lease<Data=u8, Meta=BufHeader<M>>,
          M: TryClone,
{
    #[inline]
    fn new_resident_layout(lease: &L1) -> Layout {
        unsafe { Layout::for_array_unchecked::<u8>(CString::header(lease).len) }
    }

    #[inline]
    unsafe fn resident_stow(src: &mut L1, dst: &mut L2, _hold: &Hold<'b>) -> Result<(), HoldError> {
        let src_meta = src.meta();
        let dst_meta = dst.meta();
        ptr::copy_nonoverlapping(src_meta, dst_meta, 1);
        let len = (*dst_meta).len;
        (*dst_meta).cap = len;
        ptr::copy_nonoverlapping(src.data(), dst.data(), len);
        Ok(())
    }

    #[inline]
    unsafe fn resident_unstow(_src: &mut L1, _dst: &mut L2) {
        unimplemented!();
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> CStringLease<L, M> {
    #[inline]
    fn header(&self) -> &BufHeader<M> {
        CString::header(&self.lease)
    }

    #[inline]
    fn header_mut(&mut self) -> &mut BufHeader<M> {
        CString::header_mut(&mut self.lease)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
       CString::len(&self.lease) == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        CString::len(&self.lease)
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.header().cap
    }

    #[inline]
    pub fn meta(&self) -> &M {
        &self.header().meta
    }

    #[inline]
    pub fn meta_mut(&mut self) -> &mut M {
        &mut self.header_mut().meta
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        CString::as_slice(&self.lease)
    }

    #[inline]
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        CString::as_str(&self.lease)
    }

    #[inline]
    pub unsafe fn as_str_unchecked(&self) -> &str {
        CString::as_str_unchecked(&self.lease)
    }

    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        CString::as_cstr(&self.lease)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.lease.data()
    }

    #[inline]
    pub fn as_cptr(&self) -> *const cchar {
        self.lease.data() as *const cchar
    }

    pub fn clear(&mut self) {
        self.header_mut().len = 0;
    }
}

impl<'a, L: DynamicLease<'a, Data=u8, Meta=BufHeader<M>>, M> CStringLease<L, M> {
    pub fn try_reserve(&mut self, ext: usize) -> Result<(), HoldError> {
        let buf = unsafe { mem::transmute::<&mut CStringLease<L, M>, &mut BufLease<L, u8, M>>(self) };
        buf.try_reserve(ext)
    }

    pub fn reserve(&mut self, ext: usize) {
        self.try_reserve(ext).unwrap();
    }

    pub fn try_reserve_exact(&mut self, ext: usize) -> Result<(), HoldError> {
        let buf = unsafe { mem::transmute::<&mut CStringLease<L, M>, &mut BufLease<L, u8, M>>(self) };
        buf.try_reserve_exact(ext)
    }

    pub fn reserve_exact(&mut self, ext: usize) {
        self.try_reserve_exact(ext).unwrap();
    }

    pub fn try_reserve_in_place(&mut self, ext: usize) -> Result<(), HoldError> {
        let buf = unsafe { mem::transmute::<&mut CStringLease<L, M>, &mut BufLease<L, u8, M>>(self) };
        buf.try_reserve_in_place(ext)
    }

    pub fn try_reserve_in_place_exact(&mut self, ext: usize) -> Result<(), HoldError> {
        let buf = unsafe { mem::transmute::<&mut CStringLease<L, M>, &mut BufLease<L, u8, M>>(self) };
        buf.try_reserve_in_place_exact(ext)
    }

    pub fn try_push(&mut self, c: char) -> Result<(), HoldError> {
        unsafe {
            let mut bytes = [0u8; 4];
            let n = c.encode_utf8(&mut bytes).len();
            self.try_reserve(n)?;
            let header = self.lease.meta();
            let mut len = (*header).len;
            if len != 0 { len = len.wrapping_sub(1); }
            let data = self.lease.data().wrapping_add(len);
            ptr::copy_nonoverlapping(bytes.as_ptr(), data, n);
            let data = data.wrapping_add(n);
            ptr::write(data, 0);
            (*header).len = len.wrapping_add(n).wrapping_add(1);
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
            let header = self.lease.meta();
            let mut len = (*header).len;
            if len != 0 { len = len.wrapping_sub(1); }
            let data = self.lease.data().wrapping_add(len);
            ptr::copy_nonoverlapping(s.as_ptr(), data, n);
            let data = data.wrapping_add(n);
            ptr::write(data, 0);
            (*header).len = len.wrapping_add(n).wrapping_add(1);
            Ok(())
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.try_push_str(s).unwrap();
    }
}

impl<L: Lease<Data=u8, Meta=BufHeader<M>>, M> Deref for CStringLease<L, M> {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &CStr {
        self.as_cstr()
    }
}

impl<'a, L: DynamicLease<'a, Data=u8, Meta=BufHeader<M>>, M> Write for CStringLease<L, M> {
    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        match self.try_push(c) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }

    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.try_push_str(s) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}
