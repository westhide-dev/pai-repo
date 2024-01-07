use crate::scanner::{
    comment::Comment, ident::Ident, keyword::Keyword, lit::Lit, punctuator::Punctuator,
};

/// Unit enum has the same representation as
/// ```rust
/// #[repr(u8)]
/// #[derive(Copy, Clone)]
/// enum EnumDiscriminant {
///     UID,
/// }
///
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct UnitVariant<T: Copy>(EnumDiscriminant, T);
///
/// #[repr(C)]
/// union UnitRepr<T: Copy> {
///     Item: UnitVariant<T>,
/// }
/// ```
///
/// Read more about Primitive representations:
/// https://doc.rust-lang.org/reference/type-layout.html#primitive-representations
/// https://doc.rust-lang.org/std/mem/fn.discriminant.html
#[derive(Debug)]
#[repr(u8)]
pub enum Unit<'s> {
    Keyword(Keyword),

    /// Punctuator
    Punctuator(Punctuator),

    Ident(Ident<'s>),

    /// literal
    Lit(Lit<'s>),

    Comment(Comment<'s>),
}

impl<'s> Unit<'s> {
    pub fn uid(&self) -> u8 {
        std::mem::discriminant(self);
        // Safety:
        // Unit store `u8` discriminant as its first field, so we can read the discriminant as unit id.
        unsafe { *<*const Self>::from(self).cast::<u8>() }
    }
}
