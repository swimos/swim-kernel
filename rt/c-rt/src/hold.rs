use swim_mem::alloc::Hold;
use swim_c::stdlib::MallocHold;

#[no_mangle]
unsafe extern "Rust" fn _swim_global_hold<'a>() -> &'a dyn Hold<'a> {
    MallocHold::global()
}
