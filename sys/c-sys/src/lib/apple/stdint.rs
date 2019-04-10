// ISO/IEC 9899
// 7.18 Integer types

// 7.18.1.1 Exact-width integer types
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

// 7.18.1.2 Minimum-width integer types
pub type int_least8_t = int8_t;
pub type int_least16_t = int16_t;
pub type int_least32_t = int32_t;
pub type int_least64_t = int64_t;
pub type uint_least8_t = uint8_t;
pub type uint_least16_t = uint16_t;
pub type uint_least32_t = uint32_t;
pub type uint_least64_t = uint64_t;

// 7.18.1.3 Fastest-width integer types
pub type int_fast8_t = int8_t;
pub type int_fast16_t = int16_t;
pub type int_fast32_t = int32_t;
pub type int_fast64_t = int64_t;
pub type uint_fast8_t = uint8_t;
pub type uint_fast16_t = uint16_t;
pub type uint_fast32_t = uint32_t;
pub type uint_fast64_t = uint64_t;

// 7.18.1.4 Integer types capable of holding object pointers
pub type intptr_t = isize;
pub type uintptr_t = usize;

// 7.18.1.5 Greatest-width integer types
pub type intmax_t = i64;
pub type uintmax_t = u64;
