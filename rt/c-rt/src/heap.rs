use swim_mem::alloc::Heap;
use swim_c::stdlib::MallocHeap;

#[no_mangle]
unsafe extern "Rust" fn _swim_global_heap<'a>() -> &'a dyn Heap<'a> {
    MallocHeap::global()
}
