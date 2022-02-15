#![cfg_attr(feature = "marker_trait_attr", feature(marker_trait_attr))]
#![cfg_attr(feature = "never_type", feature(never_type))]
#![no_std]

pub use {prim_signed_int::PrimSignedInt, prim_unsigned_int::PrimUnsignedInt};

/// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
/// for [primitive integers](https://doc.rust-lang.org/reference/types/numeric.html#integer-types)
#[cfg_attr(feature = "marker_trait_attr", marker)]
pub trait PrimInt: PrimNum {}

#[cfg(not(feature = "marker_trait_attr"))]
mod prim_int_impl {
    impl super::PrimInt for u8 {}
    impl super::PrimInt for u16 {}
    impl super::PrimInt for u32 {}
    impl super::PrimInt for u64 {}
    impl super::PrimInt for u128 {}
    impl super::PrimInt for usize {}

    impl super::PrimInt for i8 {}
    impl super::PrimInt for i16 {}
    impl super::PrimInt for i32 {}
    impl super::PrimInt for i64 {}
    impl super::PrimInt for i128 {}
    impl super::PrimInt for isize {}
}

#[cfg(feature = "marker_trait_attr")]
mod prim_int_impl {
    impl<T: super::PrimUnsignedInt> super::PrimInt for T {}
    impl<T: super::PrimSignedInt> super::PrimInt for T {}
}

mod prim_unsigned_int {
    /// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
    /// for [primitive unsigned integers](https://doc.rust-lang.org/reference/types/numeric.html#integer-types)
    #[cfg_attr(feature = "marker_trait_attr", marker)]
    pub trait PrimUnsignedInt: super::PrimInt {}

    impl PrimUnsignedInt for u8 {}
    impl PrimUnsignedInt for u16 {}
    impl PrimUnsignedInt for u32 {}
    impl PrimUnsignedInt for u64 {}
    impl PrimUnsignedInt for u128 {}
    impl PrimUnsignedInt for usize {}
}

mod prim_signed_int {
    /// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
    /// for [primitive signed integers](https://doc.rust-lang.org/reference/types/numeric.html#integer-types)
    #[cfg_attr(feature = "marker_trait_attr", marker)]
    pub trait PrimSignedInt: super::PrimInt {}

    impl PrimSignedInt for i8 {}
    impl PrimSignedInt for i16 {}
    impl PrimSignedInt for i32 {}
    impl PrimSignedInt for i64 {}
    impl PrimSignedInt for i128 {}
    impl PrimSignedInt for isize {}
}

/// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
/// for [primitive floating point types](https://doc.rust-lang.org/reference/types/numeric.html#floating-point-types)
#[cfg_attr(feature = "marker_trait_attr", marker)]
pub trait PrimFloat: PrimNum {}

#[cfg(feature = "marker_trait_attr")]
mod prim_num_impl {
    impl<T: crate::PrimFloat> crate::PrimNum for T {}
    impl<T: crate::PrimInt> crate::PrimNum for T {}
}

mod prim_float_impl {
    impl crate::PrimFloat for f32 {}
    impl crate::PrimFloat for f64 {}
}

/// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
/// for [primitive numeric types](https://doc.rust-lang.org/reference/types/numeric.html).
#[cfg_attr(feature = "marker_trait_attr", marker)]
pub trait PrimNum: Prim {}

#[cfg(not(feature = "marker_trait_attr"))]
mod prim_num_impl {
    impl crate::PrimNum for u8 {}
    impl crate::PrimNum for u16 {}
    impl crate::PrimNum for u32 {}
    impl crate::PrimNum for u64 {}
    impl crate::PrimNum for u128 {}
    impl crate::PrimNum for usize {}

    impl crate::PrimNum for i8 {}
    impl crate::PrimNum for i16 {}
    impl crate::PrimNum for i32 {}
    impl crate::PrimNum for i64 {}
    impl crate::PrimNum for i128 {}
    impl crate::PrimNum for isize {}

    impl crate::PrimNum for f32 {}
    impl crate::PrimNum for f64 {}
}

/// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
/// for [primitive textual types](https://doc.rust-lang.org/reference/types/textual.html)
#[cfg_attr(feature = "marker_trait_attr", marker)]
pub trait PrimTextual {}

impl PrimTextual for char {}
impl PrimTextual for str {}

/// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
/// for [primitive types](https://doc.rust-lang.org/reference/types.html)
#[cfg_attr(feature = "marker_trait_attr", marker)]
pub trait Prim {}

#[cfg(not(feature = "marker_trait_attr"))]
mod prim_impl {
    impl crate::Prim for bool {}

    impl crate::Prim for u8 {}
    impl crate::Prim for u16 {}
    impl crate::Prim for u32 {}
    impl crate::Prim for u64 {}
    impl crate::Prim for u128 {}
    impl crate::Prim for usize {}

    impl crate::Prim for i8 {}
    impl crate::Prim for i16 {}
    impl crate::Prim for i32 {}
    impl crate::Prim for i64 {}
    impl crate::Prim for i128 {}
    impl crate::Prim for isize {}

    impl crate::Prim for f32 {}
    impl crate::Prim for f64 {}

    impl crate::Prim for char {}
    impl crate::Prim for str {}

    #[cfg(feature = "never_type")]
    impl crate::Prim for ! {}
}

#[cfg(feature = "marker_trait_attr")]
mod prim_impl {
    impl crate::Prim for bool {}
    impl<T: crate::PrimNum> crate::Prim for T {}
    impl<T: crate::PrimTextual> crate::Prim for T {}

    #[cfg(feature = "never_type")]
    impl crate::Prim for ! {}
}
