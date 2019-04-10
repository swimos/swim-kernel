extern crate swim_mem;
extern crate swim_c;

use swim_mem::alloc::Pool;
use swim_mem::lease::RawBox;
use swim_c::stdlib::MallocHeap;

#[test]
fn test_malloc_pool_alloc_dealloc_boxes() {
    let heap = MallocHeap::new(4096);
    let pool = &Pool::new(&heap);

    assert_eq!(pool.live(), 0);
    assert_eq!(pool.used(), 0);
    {
        let x = RawBox::hold_new(pool, 5usize);
        assert_eq!(pool.live(), 1);
        assert_eq!(pool.used(), 8);
        assert_eq!(*x, 5);
        {
            let y = RawBox::hold_new(pool, 9usize);
            assert_eq!(pool.live(), 2);
            assert_eq!(pool.used(), 16);
            assert_eq!(*x, 5);
            assert_eq!(*y, 9);
        }
        assert_eq!(pool.live(), 1);
        assert_eq!(pool.used(), 8);
        assert_eq!(*x, 5);
    }
    assert_eq!(pool.live(), 0);
    assert_eq!(pool.used(), 0);
}
