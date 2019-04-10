use core::marker::PhantomData;
use core::ptr::NonNull;
use core::raw::TraitObject;

/// The vtable of a type with a `Reified` field as its first struct member,
/// from which a trait object can be constructed from a thin pointer to the
/// base address of the object.
///
/// # Safety
///
/// Can only reify a type that has a `Reified` field as its first struct member.
#[repr(C)]
pub struct Reified<T: ?Sized> {
    /// Pointer to the vtable of the concrete type of the object whose base
    /// address equals the address of this `Reified` structure.
    vtable: NonNull<()>,
    marker: PhantomData<*const T>,
}

impl<T: ?Sized> Reified<T> {
    /// Returns a new `Reified` structure with an uninitialized vtable.
    #[inline]
    pub const unsafe fn uninitialized() -> Reified<T> {
        Reified {
            vtable: NonNull::new_unchecked(1 as *mut ()),
            marker: PhantomData,
        }
    }

    /// Initializes the vtable of the `Reified` structure, which resides at the
    /// base address of the referenced `object`, to point to the vtable of the
    /// passed-in trait object.
    ///
    /// # Safety
    ///
    /// Assumes that the concrete `object` type has a `Reified` field as its
    /// first struct member.
    #[inline]
    pub unsafe fn deify(object: TraitObject) {
        let base = object.data as *mut Reified<T>;
        (*base).vtable = NonNull::new_unchecked(object.vtable);
    }

    /// Returns a trait object for the concrete type of the object whose base
    /// address equals the address of the passed-in `Reified` reference.
    ///
    /// # Safety
    ///
    /// Assumes that the address of the passed-in `Reified` reference equals
    /// the address of an object, and that the concrete object type's vtable
    /// pointer equals the vtable pointer contained in the `Reified` structure.
    #[inline]
    pub unsafe fn reify(&self) -> TraitObject {
        TraitObject {
            data: self as *const Reified<T> as *mut (),
            vtable: self.vtable.as_ptr(),
        }
    }
}

/// A type with a `Reified` field as its first struct member, from which a
/// polymorphic trait object can be constructed from a thin pointer to the
/// base address of the object.
///
/// # Safety
///
/// Can only reify a type that has a `Reified` field as its first struct member.
pub trait Reify<'a, T: ?Sized = Self> {
    /// Initialized the `Reified` structure at the base address of the `object`
    /// reference to the vtable of the `object` instance.
    unsafe fn deify(object: &mut T);

    /// Returns a polymorphic trait object for the concrete type of the object
    /// whose base address equals the address of the passed-in `Reified`
    /// reference.
    unsafe fn reify(base: &'a Reified<T>) -> &'a T;
}
