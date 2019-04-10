extern crate swim_mem;
extern crate swim_c;

use swim_mem::lease::RawBox;
use swim_c::stdlib::MallocHold;

#[test]
fn test_malloc_hold_alloc_dealloc_boxes() {
    let hold = &*MallocHold::new().unwrap();

    assert_eq!(hold.live(), 0);
    assert_eq!(hold.used(), 0);
    {
        let x = RawBox::hold_new(hold, 5usize);
        assert_eq!(hold.live(), 1);
        assert_eq!(hold.used(), 8);
        assert_eq!(*x, 5);
        {
            let y = RawBox::hold_new(hold, 9usize);
            assert_eq!(hold.live(), 2);
            assert_eq!(hold.used(), 16);
            assert_eq!(*x, 5);
            assert_eq!(*y, 9);
        }
        assert_eq!(hold.live(), 1);
        assert_eq!(hold.used(), 8);
        assert_eq!(*x, 5);
    }
    assert_eq!(hold.live(), 0);
    assert_eq!(hold.used(), 0);
}
