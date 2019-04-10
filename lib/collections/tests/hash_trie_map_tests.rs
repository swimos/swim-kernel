extern crate swim_mem;
extern crate swim_collections;

mod hash_collision;

use swim_mem::block::Block;
use swim_mem::alloc::{Slab, Pool};
use swim_collections::hash_trie::HashTrieMap;
use crate::hash_collision::HashCollision;

macro_rules! test_insert {
    ($empty:expr, $n:expr, $apply:expr) => ({
        let mut xs = $empty;
        let n = $n;
        for i in 0..n {
            let result = xs.insert($apply(i), i);
            assert_eq!(result, Ok(None));
            assert_eq!(xs.len(), (i + 1) as usize);
            assert!(xs.contains_key(&$apply(i)));
            assert_eq!(xs.get(&$apply(i)), Some(&i));
        }
        for i in 0..n {
            assert!(xs.contains_key(&$apply(i)));
            assert_eq!(xs.get(&$apply(i)), Some(&i));
        }
        for i in n..2*n {
            assert!(!xs.contains_key(&$apply(i)));
            assert_eq!(xs.get(&$apply(i)), None);
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
            let result = xs.insert($apply(i), -i);
            assert_eq!(result, Ok(None));
            assert_eq!(xs.len(), (i + 1) as usize);
            assert!(xs.contains_key(&$apply(i)));
            assert_eq!(xs.get(&$apply(i)), Some(&-i));
            let result = xs.insert($apply(i), i);
            assert_eq!(result, Ok(Some(-i)));
            assert_eq!(xs.len(), (i + 1) as usize);
            assert!(xs.contains_key(&$apply(i)));
            assert_eq!(xs.get(&$apply(i)), Some(&i));
        }
        for i in 0..n {
            assert!(xs.contains_key(&$apply(i)));
            assert_eq!(xs.get(&$apply(i)), Some(&i));
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
            xs.insert($apply(i), i).unwrap();
        }
        for i in (0..n).rev() {
            let result = xs.remove(&$apply(i));
            assert_eq!(result, Ok(Some(i)));
            assert!(!xs.contains_key(&$apply(i)));
            assert_eq!(xs.len(), i as usize);
            if i < 1 << 10 {
                for j in 0..i {
                    assert!(xs.contains_key(&$apply(j)));
                }
                for j in i..n {
                    assert!(!xs.contains_key(&$apply(j)));
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
            xs.insert($apply(i), i).unwrap();
        }
        for i in (0..n).rev() {
            let result = xs.remove(&$apply(i));
            assert_eq!(result, Ok(Some(i)));
            assert_eq!(xs.len(), i as usize);
            assert!(!xs.contains_key(&$apply(i)));
            let result = xs.remove(&$apply(i));
            assert_eq!(result, Ok(None));
            assert_eq!(xs.len(), i as usize);
            assert!(!xs.contains_key(&$apply(i)));
        }
    });
    ($empty:expr, $n:expr) => (
        test_reremove!($empty, $n, |i| i);
    );
}

macro_rules! test_iter {
    ($empty:expr, $n:expr, $i:ident => $apply:expr, $k:ident => $unapply:expr) => ({
        let mut xs = $empty;
        assert_eq!(xs.iter().next(), None);
        let n = $n;
        for i in 1..(n+1) {
            let $i = i;
            xs.insert($apply, i).unwrap();
            let mut key_sum = 0;
            let mut val_sum = 0;
            for (key, value) in xs.iter() {
                let $k = key;
                key_sum += $unapply;
                val_sum += *value;
            }
            assert_eq!(key_sum, i * (i + 1) / 2);
            assert_eq!(val_sum, i * (i + 1) / 2);
        }
    });
    ($empty:expr, $n:expr) => (
        test_iter!($empty, $n, i => i, k => *k);
    );
}

#[test]
fn test_hash_trie_map_insert() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_insert!(HashTrieMap::<usize, usize>::hold_new(pool), 1 << 15);
}

#[test]
fn test_hash_trie_map_reinsert() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_reinsert!(HashTrieMap::<isize, isize>::hold_new(pool), 1 << 15);
}

#[test]
fn test_hash_trie_map_remove() {
    static mut TEST_HUNK: [u8; 16*1024*1024] = [0; 16*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_remove!(HashTrieMap::<usize, usize>::hold_new(pool), 1 << 15);
}

#[test]
fn test_hash_trie_map_reremove() {
    static mut TEST_HUNK: [u8; 16*1024*1024] = [0; 16*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_reremove!(HashTrieMap::<usize, usize>::hold_new(pool), 1 << 15);
}

#[test]
fn test_hash_trie_map_iter() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_iter!(HashTrieMap::<usize, usize>::hold_new(pool), 1 << 10);
}

#[test]
fn test_hash_trie_map_insert_collisions() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_insert!(HashTrieMap::<HashCollision<usize>, usize>::hold_new(pool), 1 << 15,
                 |i| HashCollision::new(i, i >> 2));
}

#[test]
fn test_hash_trie_map_reinsert_collisions() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_reinsert!(HashTrieMap::<HashCollision<isize>, isize>::hold_new(pool), 1 << 15,
                   |i| HashCollision::new(i, i >> 2));
}

#[test]
fn test_hash_trie_map_remove_collisions() {
    static mut TEST_HUNK: [u8; 16*1024*1024] = [0; 16*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_remove!(HashTrieMap::<HashCollision<usize>, usize>::hold_new(pool), 1 << 15,
                 |i| HashCollision::new(i, i >> 2));
}

#[test]
fn test_hash_trie_map_reremove_collisions() {
    static mut TEST_HUNK: [u8; 16*1024*1024] = [0; 16*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_reremove!(HashTrieMap::<HashCollision<usize>, usize>::hold_new(pool), 1 << 15,
                   |i| HashCollision::new(i, i >> 2));
}

#[test]
fn test_hash_trie_map_iter_collisions() {
    static mut TEST_HUNK: [u8; 8*1024*1024] = [0; 8*1024*1024];
    let slab = Slab::new(unsafe { Block::from_slice(&mut TEST_HUNK) }, 4096);
    let pool = &Pool::new(&slab);

    test_iter!(HashTrieMap::<HashCollision<usize>, usize>::hold_new(pool), 1 << 10,
               i => HashCollision::new(i, i >> 2),
               key => **key);
}
