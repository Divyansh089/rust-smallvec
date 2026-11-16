use core::{
    fmt::{
        Debug,
        Formatter,
        Result as Format
    },
    marker::PhantomData
};
#[cfg(feature = "defmt")]
use defmt::{
    Format as DeFormat,
    Formatter as DeFormatter,
    write as dewrite
};

/// Vec guarantees that its length is always less than [`isize::MAX`] in
/// *bytes*.
///
/// For a non-ZST, this means that the length is less than `isize::MAX` objects,
/// which implies we have at least one free bit we can use. We use the least
/// significant bit for the tag. And store the length in the `usize::BITS - 1`
/// most significant bits.
///
/// For a ZST, we never use the heap, so we just store the length directly.
#[repr(transparent)]
pub struct LocatedLength<Item>(usize, PhantomData<Item>);

// We don't use `#[derive(Clone, Copy)]` instead because `Item` doesn't need to
// be `Copy` or `Clone`.
impl<Item> Clone for LocatedLength<Item> {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}

impl<Item> Copy for LocatedLength<Item> {}

impl<Item> Debug for LocatedLength<Item> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Format {
        let (length, spilled) = self.parts();
        f.debug_struct("LocatedLength")
            .field("length", &length)
            .field("spilled", &spilled)
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl<Item> DeFormat for LocatedLength<Item> {
    #[inline]
    fn format(&self, fmt: DeFormatter<'_>) {
        let (length, spilled) = self.parts();
        dewrite!(
            fmt,
            "LocatedLength {{ length: {=usize}, spilled: {=bool} }}",
            length,
            spilled
        );
    }
}

#[allow(clippy::len_without_is_empty)]
impl<Item> LocatedLength<Item> {
    #[cfg(not(target_pointer_width = "64"))]
    const MAX_LEN: usize = usize::MAX >> Self::SHIFT;
    const SHIFT: u32 = (size_of::<Item>() != 0) as u32;
    const TAG: usize = Self::SHIFT as usize;

    /// For non-ZST `Item`, `length` must not exceed `isize::MAX`.
    /// For ZST `Item`, `spilled` must be `false`.
    #[inline]
    pub const fn new(length: usize, spilled: bool) -> Self {
        Self(
            (length << Self::SHIFT) | ((spilled as usize) & Self::TAG),
            PhantomData
        )
    }

    #[inline(always)]
    pub const fn len(self) -> usize { self.0 >> Self::SHIFT }

    #[inline(always)]
    #[must_use]
    pub const fn is_spilled(self) -> bool { self.0 & Self::TAG != 0 }

    #[inline(always)]
    pub const fn parts(self) -> (usize, bool) { (self.0 >> Self::SHIFT, (self.0 & Self::TAG) != 0) }

    #[inline]
    pub const fn set_location<const ON: bool>(&mut self) {
        if Self::TAG != 0 {
            self.0 = (self.0 & !Self::TAG) | ON as usize;
        }
    }

    /// # Safety
    ///
    /// On 64-bit targets, `self.len() + n` must not overflow and must not
    /// exceed `isize::MAX` for non-ZST `Item`, or `usize::MAX` for ZST `Item`.
    #[inline]
    pub const unsafe fn add(&mut self, n: usize) {
        #[cfg(not(target_pointer_width = "64"))]
        {
            #[cold]
            #[inline(never)]
            const fn assert_failed() { panic!("smallvec length overflow") }
            match self.len().checked_add(n) {
                Some(value) => {
                    if value > Self::MAX_LEN {
                        assert_failed()
                    }
                }
                None => assert_failed()
            }
        }
        self.0 += n << Self::SHIFT;
    }

    /// # Safety
    ///
    /// current length must be greater equal than n
    #[inline(always)]
    pub const unsafe fn sub(&mut self, n: usize) { self.0 -= n << Self::SHIFT; }
}
