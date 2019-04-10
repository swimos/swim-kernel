#[macro_export]
macro_rules! cfg_if {
    (
        $(if #[cfg($($meta:meta),*)] {
            $($item1:item)* })
        else * else {
            $($item2:item)*
        }
    ) => {
        __cfg_if_items! {
            () ;
            $( ( ($($meta),*) ($($item1)*) ), )*
            ( () ($($item2)*) ),
        }
    };
    (
        if #[cfg($($if_meta:meta),*)] {
            $($if_item:item)*
        }
        $(
            else if #[cfg($($else_meta:meta),*)] {
                $($else_item:item)*
            }
        )*
    ) => {
        __cfg_if_items! {
            () ;
            ( ($($if_meta),*) ($($if_item)*) ),
            $( ( ($($else_meta),*) ($($else_item)*) ), )*
            ( () () ),
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __cfg_if_items {
    (($($not:meta,)*) ; ) => {};
    (($($not:meta,)*) ; ( ($($meta:meta),*) ($($item:item)*) ), $($rest:tt)*) => {
        __cfg_if_apply! { cfg(all($($meta,)* not(any($($not),*)))), $($item)* }
        __cfg_if_items! { ($($not,)* $($meta,)*) ; $($rest)* }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __cfg_if_apply {
    ($meta:meta, $($item:item)*) => {
        $(#[$meta] $item)*
    }
}
