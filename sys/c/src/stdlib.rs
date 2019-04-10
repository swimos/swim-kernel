use core::cmp;
use core::marker::PhantomData;
use core::mem;
use core::ptr;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;
use swim_core::reify::{Reified, Reify};
use swim_mem::block::{Block, Layout, ZSP};
use swim_mem::alloc::{AllocTag, Heap, HeapError, Hold, HoldError};
use swim_mem::lease::RawBox;
use swim_c_sys::void;

pub use swim_c_sys::stdlib::*;

const MALLOC_HEAP_UNIT: usize = 4096;

/// Block allocator backed by libc `malloc`.
pub struct MallocHeap<'a> {
    /// Minimum number of bytes in each memory block.
    unit: usize,
    /// Number of currently allocated memory blocks.
    live: AtomicUsize,
    /// Lifetime of allocated memory blocks.
    lifetime: PhantomData<&'a ()>,
}

unsafe impl<'a> Send for MallocHeap<'a> {
}

unsafe impl<'a> Sync for MallocHeap<'a> {
}

impl<'a> MallocHeap<'a> {
    /// Returns a new `Heap` that allocates memory blocks with at least `unit`
    /// size bytes using libc `malloc`.
    pub const fn new(unit: usize) -> MallocHeap<'a> {
        MallocHeap {
            unit: unit,
            live: AtomicUsize::new(0),
            lifetime: PhantomData,
        }
    }

    /// Returns a reference to a `Heap` that allocates memory using libc `malloc`.
    pub fn global() -> &'a MallocHeap<'a> {
        // Declare the global malloc heap singleton with an uninitialized vtable.
        // Rust apparently has no way to get a static pointer to a vtable.
        static GLOBAL: MallocHeap<'static> = MallocHeap {
            unit: MALLOC_HEAP_UNIT,
            live: AtomicUsize::new(0),
            lifetime: PhantomData,
        };
        unsafe {
            // Return a reference to the global malloc heap.
            mem::transmute(&GLOBAL)
        }
    }

    /// Returns the minimum number of bytes in each memory block in this `MallocHeap`.
    #[inline]
    pub fn block_size(&self) -> usize {
        self.unit as usize
    }

    /// Returns the number of currently allocated memory blocks in this `MallocHeap`.
    #[inline]
    pub fn live(&self) -> usize {
        self.live.load(Relaxed) as usize
    }
}

impl<'a> Heap<'a> for MallocHeap<'a> {
    unsafe fn alloc(&self, layout: Layout) -> Result<Block<'a>, HeapError> {
        let ptr;
        let size;
        if layout.size() != 0 {
            size = cmp::max(layout.size(), self.unit);
            ptr = malloc(size) as *mut u8;
            if ptr.is_null() {
                return Err(HeapError::OutOfMemory);
            }
            if ptr as usize % layout.align() != 0 {
                free(ptr as *mut void);
                return Err(HeapError::Misaligned);
            }
        } else {
            ptr = ZSP;
            size = 0;
        };
        // Increment the live block count.
        self.live.fetch_add(1, Relaxed);
        Ok(Block::from_raw_parts(ptr, size))
    }

    unsafe fn dealloc(&self, block: Block<'a>) -> usize {
        let ptr = block.as_ptr();
        if ptr != ZSP {
            free(ptr as *mut void);
        }
        // Decrement the live block count.
        self.live.fetch_sub(1, Relaxed);
        block.size()
    }
}

/// Managed allocator backed by libc `malloc`.
pub struct MallocHold<'a> {
    /// Polymorphic hold type.
    base: Reified<Hold<'a>>,
    /// Number of currently allocated memory blocks.
    live: AtomicUsize,
    /// Number of currently allocated bytes.
    used: AtomicUsize,
    /// Tag shared by all zero-sized allocations.
    zero: AllocTag<'a>,
}

unsafe impl<'a> Send for MallocHold<'a> {
}

unsafe impl<'a> Sync for MallocHold<'a> {
}

impl<'a> MallocHold<'a> {
    /// Returns an owned reference to a new `Hold` that allocates memory blocks
    /// using libc `malloc`.
    pub fn new() -> Result<RawBox<'a, MallocHold<'a>>, HoldError> {
        // Allocate a new malloc hold on the heap using the global malloc hold.
        let mut hold_box = RawBox::try_hold_new(MallocHold::global(), MallocHold {
            base: unsafe { Reified::uninitialized() },
            live: AtomicUsize::new(0),
            used: AtomicUsize::new(0),
            zero: unsafe { AllocTag::null() },
        })?;
        unsafe {
            // Borrow a mutable reference to the malloc hold.
            let hold = hold_box.as_mut();
            // Initialize the malloc hold's vtable (idempotent).
            MallocHold::deify(hold);
            // Initialize the malloc hold's shared zero tag (idempotent).
            hold.zero.init(&hold.base);
        }
        Ok(hold_box)
    }

    /// Returns a reference to a `Hold` that allocates memory using libc `malloc`.
    pub fn global() -> &'a MallocHold<'a> {
        // Declare the global malloc heap singleton with an uninitialized vtable.
        // Rust apparently has no way to get a static pointer to a vtable.
        static mut GLOBAL: MallocHold<'static> = MallocHold {
            base: unsafe { Reified::uninitialized() },
            live: AtomicUsize::new(0),
            used: AtomicUsize::new(0),
            zero: unsafe { AllocTag::null() },
        };
        unsafe {
            // Initialize the global malloc hold's vtable (idempotent).
            MallocHold::deify(&mut GLOBAL);
            // Initialize the global malloc hold's shared zero tag (idempotent).
            GLOBAL.zero.init(&GLOBAL.base);
            // Return a reference to the global malloc hold.
            mem::transmute(&GLOBAL)
        }
    }

    /// Returns the number of currently allocated memory blocks in this `MallocHold`.
    #[inline]
    pub fn live(&self) -> usize {
        self.live.load(Relaxed)
    }

    /// Returns the number of bytes currently allocated in this `MallocHold`.
    #[inline]
    pub fn used(&self) -> usize {
        self.used.load(Relaxed)
    }
}

unsafe impl<'a> Hold<'a> for MallocHold<'a> {
    unsafe fn alloc(&self, layout: Layout) -> Result<Block<'a>, HoldError> {
        // Get the size of the block to allocate.
        let new_size = layout.size();
        // Pointer to the newly allocated block.
        let new_ptr;
        // Check if the layout has non-zero size.
        if new_size != 0 {
            // Get the layout of the allocation tag.
            let tag_layout = Layout::for_type::<AllocTag<'a>>();
            // Lay out the box structure with an allocation tag preceding the block.
            let (box_layout, block_offset) = tag_layout.extended(layout)?;
            // Allocate the box structure on the heap.
            let ptr = malloc(box_layout.size()) as *mut u8;
            // Check if the allocation succeeded.
            if ptr.is_null() {
                return Err(HoldError::OutOfMemory);
            }
            // Get a pointer to the tag at the base address of the allocation.
            let tag_ptr = ptr as *mut AllocTag<'a>;
            // Get a pointer to the block that follows the allocation tag.
            new_ptr = (ptr as usize).wrapping_add(block_offset) as *mut u8;
            // Initialize the allocation tag.
            ptr::write(tag_ptr, AllocTag::new(&self.base));
            // Increase the allocated byte count.
            self.used.fetch_add(new_size, Relaxed);
        } else {
            // Get the address of the zero-sized allocation tag.
            let tag_ptr = &self.zero as *const AllocTag<'a>;
            // Get the address of the zero-sized block immediately following the tag.
            new_ptr = (tag_ptr as usize).wrapping_add(mem::size_of::<AllocTag<'a>>()) as *mut u8;
        }
        // Increment the live block count.
        self.live.fetch_add(1, Relaxed);
        // Return the allocated block.
        Ok(Block::from_raw_parts(new_ptr, new_size))
    }

    unsafe fn dealloc(&self, block: Block<'a>) -> usize {
        // Get the size of the allocated block.
        let size = block.size();
        // Check if the block has non-zero size.
        if size != 0 {
            // Get the size of the allocation tag.
            let tag_size = mem::size_of::<AllocTag>();
            // Get a pointer to the base address of the allocation.
            let ptr = (block.as_ptr() as usize).wrapping_sub(tag_size) as *mut u8;
            // Free the allocation.
            free(ptr as *mut void);
            // Decrease the allocated byte count.
            self.used.fetch_sub(size, Relaxed);
        }
        // Decrement the live allocation count.
        self.live.fetch_sub(1, Relaxed);
        // Return the number of freed bytes.
        size
    }

    unsafe fn resize(&self, block: Block<'a>, layout: Layout) -> Result<Block<'a>, HoldError> {
        self.realloc(block, layout)
    }

    unsafe fn realloc(&self, block: Block<'a>, layout: Layout) -> Result<Block<'a>, HoldError> {
        // Get the layout of the allocation tag.
        let tag_layout = Layout::for_type::<AllocTag<'a>>();
        // Get the size of the current block.
        let old_size = block.size();
        // Get the size of the new block to allocate.
        let new_size = layout.size();
        // Compute the size difference between the old block and the new block.
        let size_diff = (new_size as isize).wrapping_sub(old_size as isize);
        // Pointer to the newly allocated block.
        let new_ptr;
        if size_diff != 0 {
            // Pointer to the base address of the current allocation.
            let old_ptr;
            // Check if the old block has non-zero size.
            if old_size != 0 {
                // Get a pointer to the base address of the current allocation.
                old_ptr = (block.as_ptr() as usize).wrapping_sub(tag_layout.size()) as *mut u8;
            } else {
                // Cause realloc to behave like malloc.
                old_ptr = ptr::null_mut();
            }
            // Check if the new layout has non-zero size.
            if new_size != 0 {
                // Lay out the new box structure with an allocation tag preceding the new block.
                let (box_layout, block_offset) = tag_layout.extended(layout)?;
                // Reallocate the box structure on the heap.
                let ptr = realloc(old_ptr as *mut void, box_layout.size()) as *mut u8;
                // Check if the allocation failed.
                if ptr.is_null() {
                    // Return an out of memory error.
                    return Err(HoldError::OutOfMemory);
                }
                // Get a pointer to the tag at the base address of the allocation.
                let tag_ptr = ptr as *mut AllocTag<'a>;
                // Get a pointer to the new block that follows the allocation tag.
                new_ptr = (ptr as usize).wrapping_add(block_offset) as *mut u8;
                // Initialize the allocation tag.
                ptr::write(tag_ptr, AllocTag::new(&self.base));
            } else {
                // Check if the old block has non-zero size.
                if !old_ptr.is_null() {
                    // Free the old allocation.
                    free(old_ptr as *mut void);
                }
                // Get the address of the zero-sized allocation tag.
                let tag_ptr = &self.zero as *const AllocTag<'a>;
                // Get the address of the zero-sized block immediately following the tag.
                new_ptr = (tag_ptr as usize).wrapping_add(mem::size_of::<AllocTag<'a>>()) as *mut u8;
            }
            // Adjust the allocated byte count by the size difference.
            if size_diff > 0 {
                self.used.fetch_add(size_diff as usize, Relaxed);
            } else if size_diff < 0 {
                self.used.fetch_sub(-size_diff as usize, Relaxed);
            }
        } else {
            new_ptr = block.as_ptr();
        }
        // Return the allocated block.
        Ok(Block::from_raw_parts(new_ptr, new_size))
    }
}

impl<'a> Reify<'a, Hold<'a> + 'a> for MallocHold<'a> {
    #[inline]
    unsafe fn deify(object: &mut (Hold<'a> + 'a)) {
        Reified::<Hold<'a>>::deify(mem::transmute(object));
    }

    #[inline]
    unsafe fn reify(base: &'a Reified<Hold<'a> + 'a>) -> &'a (Hold<'a> + 'a) {
        mem::transmute(base.reify())
    }
}
