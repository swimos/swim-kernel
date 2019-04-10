#![feature(test)]

extern crate test;
extern crate swim_mem;
extern crate swim_c;

use test::Bencher;
use swim_mem::lease::{RawBox, RawBuf};
use swim_c::stdlib::MallocHold;

#[bench]
fn bench_malloc_hold_alloc_dealloc(bench: &mut Bencher) {
    let hold = MallocHold::global();
    let mut n: usize = 0;
    bench.iter(|| {
        let x = RawBox::hold_new(hold, n);
        n = n.wrapping_add(*x);
    });
}

#[bench]
fn bench_malloc_hold_alloc_dealloc_1mib(bench: &mut Bencher) {
    let hold = MallocHold::global();
    let mut k: usize = 0;
    bench.iter(|| {
        k = 0;
        let mut n: usize = 0;
        let mut x = RawBox::hold_new(hold, n);
        while k < 32768 {
            k = k.wrapping_add(1);
            n = n.wrapping_add(*x);
            x = RawBox::hold_new(hold, n);
        }
    });
}

#[bench]
fn bench_malloc_hold_alloc_dealloc_bufs(bench: &mut Bencher) {
    let hold = MallocHold::global();
    let mut k: usize = 0;
    bench.iter(|| {
        k = 0;
        let mut n: usize = 1;
        let mut _x = RawBuf::<usize>::hold_cap(hold, n);
        while k < 32768 {
            k = k.wrapping_add(1);
            n = n.wrapping_add(1) % 32;
            _x = RawBuf::hold_cap(hold, n);
        }
    });
}
