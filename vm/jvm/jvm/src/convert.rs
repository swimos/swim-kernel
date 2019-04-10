use crate::error::Result;
use crate::env::JEnv;

pub trait FromJava<T> {
    fn from_java(env: &JEnv, value: T) -> Self;
}

pub trait IntoJava<T> {
    fn into_java(self, env: &JEnv) -> T;
}

pub trait TryFromJava<T>: Sized {
    fn try_from_java(env: &JEnv, value: T) -> Result<Self>;
}

pub trait TryIntoJava<T>: Sized {
    fn try_into_java(self, env: &JEnv) -> Result<T>;
}

impl<T> FromJava<T> for T {
    #[inline]
    fn from_java(_env: &JEnv, t: T) -> Self { t }
}

impl<T> TryFromJava<T> for T {
    #[inline]
    fn try_from_java(_env: &JEnv, t: T) -> Result<Self> { Ok(t) }
}

impl<T, U> IntoJava<U> for T where U: FromJava<T> {
    #[inline]
    fn into_java(self, env: &JEnv) -> U {
        U::from_java(env, self)
    }
}

impl<T, U> TryIntoJava<U> for T where U: TryFromJava<T> {
    #[inline]
    fn try_into_java(self, env: &JEnv) -> Result<U> {
        U::try_from_java(env, self)
    }
}
