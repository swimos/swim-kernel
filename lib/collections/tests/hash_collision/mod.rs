use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Eq)]
pub struct HashCollision<T> {
    pub real: T,
    pub fake: T,
}

impl<T> HashCollision<T> {
    pub fn new(real: T, fake: T) -> HashCollision<T> {
        HashCollision {
            real: real,
            fake: fake,
        }
    }
}

impl<T> Deref for HashCollision<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.real
    }
}

impl<T> Borrow<T> for HashCollision<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.real
    }
}

impl<T: Eq> PartialEq for HashCollision<T> {
    fn eq(&self, fake: &HashCollision<T>) -> bool {
        self.real == fake.real
    }
}

impl<T: Hash> Hash for HashCollision<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fake.hash(state);
    }
}
