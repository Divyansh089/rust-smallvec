#[cfg(feature = "nightly")]
use core::ops::DerefPure as Pure;
use {
    super::SmallVec,
    core::{
        alloc::Allocator,
        borrow::{
            Borrow,
            BorrowMut
        },
        ops::{
            Deref,
            DerefMut
        },
        slice::{
            from_raw_parts,
            from_raw_parts_mut
        }
    }
};
#[cfg(feature = "stable-deref")]
use {
    alloc::alloc::Global,
    stable_deref_trait::StableDeref
};

impl<Item, const INLINE: usize, Heap: Allocator> Borrow<[Item]> for SmallVec<Item, INLINE, Heap> {
    #[inline]
    fn borrow(&self) -> &[Item] { self.as_ref() }
}

impl<Item, const INLINE: usize, Heap: Allocator> BorrowMut<[Item]>
    for SmallVec<Item, INLINE, Heap>
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut [Item] { self.as_mut() }
}

impl<Item, const INLINE: usize, Heap: Allocator> AsRef<[Item]> for SmallVec<Item, INLINE, Heap> {
    #[inline]
    fn as_ref(&self) -> &[Item] {
        let (length, spilled) = self.length.parts();
        // SAFETY: all the elements in `..length` are initialized
        unsafe { from_raw_parts(self.raw.as_ptr(spilled), length) }
    }
}

impl<Item, const INLINE: usize, Heap: Allocator> AsMut<[Item]> for SmallVec<Item, INLINE, Heap> {
    #[inline]
    fn as_mut(&mut self) -> &mut [Item] {
        let (length, spilled) = self.length.parts();
        // SAFETY: see above
        unsafe { from_raw_parts_mut(self.raw.as_mut_ptr(spilled), length) }
    }
}

impl<Item, const INLINE: usize, Heap: Allocator> Deref for SmallVec<Item, INLINE, Heap> {
    type Target = [Item];

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_ref() }
}

impl<Item, const INLINE: usize, Heap: Allocator> DerefMut for SmallVec<Item, INLINE, Heap> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
}

#[cfg(feature = "nightly")]
unsafe impl<Item, const INLINE: usize, Heap: Allocator> Pure for SmallVec<Item, INLINE, Heap> {}

#[cfg(feature = "stable-deref")]
#[cfg_attr(docsrs, doc(cfg(feature = "stable-deref")))]
// SAFETY: with no inline capacity, elements always live in a heap allocation,
// which does not move when the `SmallVec` is moved, just like `Vec`. When it is
// not spilled (empty, or holding zero-sized items) it points to a dangling
// address, also like `Vec`, which does not change either. A `SmallVec` with
// inline capacity cannot implement this, since its inline elements move
// together with it.
unsafe impl<Item> StableDeref for SmallVec<Item, 0, Global> {}
