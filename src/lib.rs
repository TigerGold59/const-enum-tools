#![feature(generic_const_exprs)]

pub trait VariantCountable {
    const VARIANT_COUNT: usize;
}

pub trait VariantIterable where
    Self: VariantCountable,
    [(); Self::VARIANT_COUNT]:,
{

    fn variant_index (&self) -> usize;

    const VARIANTS: [&'static str; Self::VARIANT_COUNT];

    fn variant_name (&self) -> String {
        Self::VARIANTS[self.variant_index()].to_string()
    }
}
