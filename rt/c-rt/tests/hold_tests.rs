extern crate swim_mem;
extern crate swim_c_rt;

use swim_mem::lease::RawBox;

#[test]
fn test_global_alloc_dealloc_boxes() {
    let x = RawBox::new(5usize);
    assert_eq!(*x, 5);
    {
        let y = RawBox::new(9usize);
        assert_eq!(*x, 5);
        assert_eq!(*y, 9);
    }
    assert_eq!(*x, 5);
}
