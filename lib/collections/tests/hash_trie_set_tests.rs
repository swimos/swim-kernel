extern crate swim_mem;
extern crate swim_collections;

mod hash_collision;

use swim_mem::block::Block;
use swim_mem::alloc::{Slab, Pool};
use swim_collections::hash_trie::HashTrieSet;
use crate::hash_collision::HashCollision;

macro_rules! test_insert {
    ($empty:expr, $n:expr, $apply:expr) => ({
        let mut xs = $empty;
        let n = $n;
        for i in 0..n {
            let result = xs.insert($apply(i));
            assert_eq!(result, Ok(true));
            assert_eq!(xs.len(), (i + 1) as usize);
            assert!(xs.contains(&$apply(i)));
        }
        for i in 0..n {
            assert!(xs.contains(&$apply(i)));
        }
        for i in n..2*n {
            assert!(!xs.contains(&$apply(i)));
        }
    });
    ($empty:expr, $n:expr) => (
        test_insert!($empty, $n, |i| i);
    );
}

macro_rules! test_reinsert {
    ($empty:expr, $n:expr, $apply:expr) => ({
        let mut xs = $empty;
        let n = $n;
        for i in 0..n {
            let result = xs.insert($apply(i));
            assert_eq!(result, Ok(true));
            assert_eq!(xs.len(), (i + 1) as usize);
            assert!(xs.contains(&$apply(i)));
            let result = xs.insert($apply(i));
            assert_eq!(result, Ok(false));
            assert_eq!(xs.len(), (i + 1) as usize);
            assert!(xs.contains(&$apply(i)));
        }
        for i in 0..n {
            assert!(xs.contains(&$apply(i)));
        }
    });
    ($empty:expr, $n:expr) => (
        test_reinsert!($empty, $n, |i| i);
    );
}

macro_rules! test_remove {
    ($empty:expr, $n:expr, $apply:expr) => ({
        let mut xs = $empty;
        let n = $n;
        for i in 0..n {
            xs.insert($apply(i)).unwrap();
        }
        for i in (0..n).rev() {
            let result = xs.remove(&$apply(i));
            assert_eq!(result, Ok(true));
            assert!(!xs.contains(&$apply(i)));
            assert_eq!(xs.len(), i as usize);
            if i < 1 << 10 {
                for j in 0..i {
                    assert!(xs.contains(&$apply(j)));
                }
                for j in i..n {
                    assert!(!xs.contains(&$apply(j)));
                }
            }
        }
    });
    ($empty:expr, $n:expr) => (
        test_remove!($empty, $n, |i| i);
    );
}

macro_rules! test_reremove {
    ($empty:expr, $n:expr, $apply:expr) => ({
        let mut xs = $empty;
        let n = $n;
        for i in 0..n {
            xs.insert($apply(i)).unwrap();
        }
        for i in (0..n).rev() {
            let result = xs.remove(&$apply(i));
            assert_eq!(result, Ok(true));
            assert_eq!(xs.len(), i as usize);
            assert!(!xs.contains(&$apply(i)));
            let result = xs.remove(&$apply(i));
            assert_eq!(result, Ok(false));
            assert_eq!(xs.len(), i as usize);
            assert!(!xs.contains(&$apply(i)));
        }
    });
    ($empty:expr, $n:expr) => (
        test_reremove!($empty, $n, |i| i);
    );
}

macro_rules! test_iter {
    ($empty:expr, $n:expr, $i:ident => $apply:expr, $e:ident => $unapply:expr) => ({
        let mut xs = $empty;
        assert_eq!(xs.iter().next(), None);
        let n = $n;
        for i in 1..(n+1) {
            let $i = i;
            xs.insert($apply).unwrap();
            let mut elem_sum = 0;
            for elem in xs.iter() {
                let $e = elem;
                elem_sum += $unapply;
            }
            assert_eq!(elem_sum, i * (i + 1) / 2);
        }
    });
    ($empty:expr, $n:expr) => (
        test_iter!($empty, $n, i => i, e => *e);
    );
}

#[test]
fn test_hash_trie_set_insert() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_insert!(HashTrieSet::<usize>::hold_new(pool), 1 << 15);
}

#[test]
fn test_hash_trie_set_reinsert() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_reinsert!(HashTrieSet::<usize>::hold_new(pool), 1 << 15);
}

#[test]
fn test_hash_trie_set_remove() {
    static mut TEST_HUNK: [u8; 16*1024*1024] = [0; 16*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_remove!(HashTrieSet::<usize>::hold_new(pool), 1 << 15);
}

#[test]
fn test_hash_trie_set_reremove() {
    static mut TEST_HUNK: [u8; 16*1024*1024] = [0; 16*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_reremove!(HashTrieSet::<usize>::hold_new(pool), 1 << 15);
}

#[test]
fn test_hash_trie_set_iter() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_iter!(HashTrieSet::<usize>::hold_new(pool), 1 << 10);
}

#[test]
fn test_hash_trie_set_insert_collisions() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_insert!(HashTrieSet::<HashCollision<usize>>::hold_new(pool), 1 << 15,
                 |i| HashCollision::new(i, i >> 2));
}

#[test]
fn test_hash_trie_set_reinsert_collisions() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_reinsert!(HashTrieSet::<HashCollision<usize>>::hold_new(pool), 1 << 15,
                   |i| HashCollision::new(i, i >> 2));
}

#[test]
fn test_hash_trie_set_remove_collisions() {
    static mut TEST_HUNK: [u8; 16*1024*1024] = [0; 16*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_remove!(HashTrieSet::<HashCollision<usize>>::hold_new(pool), 1 << 15,
                 |i| HashCollision::new(i, i >> 2));
}

#[test]
fn test_hash_trie_set_reremove_collisions() {
    static mut TEST_HUNK: [u8; 16*1024*1024] = [0; 16*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_reremove!(HashTrieSet::<HashCollision<usize>>::hold_new(pool), 1 << 15,
                   |i| HashCollision::new(i, i >> 2));
}

#[test]
fn test_hash_trie_set_iter_collisions() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_iter!(HashTrieSet::<HashCollision<usize>>::hold_new(pool), 1 << 10,
               i => HashCollision::new(i, i >> 2),
               elem => **elem);
}
