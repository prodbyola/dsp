pub mod sample {
    pub trait Sample: Sized + Copy + Default {
        fn from(&self) -> Self;
        fn into_usize(&self) -> usize;
        fn into_f64(&self) -> f64;
    }

    /// We use this macro to generate codes responsible for implementing
    /// `Sample` trait for all numeric type.
    macro_rules! impl_sample_for_numeric {
        ($($t:ty),*) => {
            $(
                impl Sample for $t {
                    fn from(&self) -> $t {
                        *self as $t
                    }

                    fn into_usize(&self) -> usize {
                        *self as usize
                    }

                    fn into_f64(&self) -> f64 {
                        *self as f64
                    }
                }
            )*
        };
    }
    
    impl_sample_for_numeric!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
    
}