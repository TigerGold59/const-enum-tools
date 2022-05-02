#![feature(generic_const_exprs)]

/// An enum trait that shows a count of its variants.
pub trait VariantCount {
    const VARIANT_COUNT: usize;
}

/// An enum trait that allows getting the index of its variant
/// and thereby the name of the variant.
pub trait VariantList where
    Self: VariantCount,
    [(); Self::VARIANT_COUNT]:,
{

    fn variant_index (&self) -> usize;

    const VARIANTS: [&'static str; Self::VARIANT_COUNT];

    fn variant_name (&self) -> String {
        Self::VARIANTS[self.variant_index()].to_string()
    }
}
