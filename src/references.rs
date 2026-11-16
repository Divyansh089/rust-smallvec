#[cfg(feature = "nightly")]
use core::ops::DerefPure as Pure;
#[cfg(feature = "stable-deref")]
use stable_deref_trait::StableDeref;
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
// SAFETY: with no inline capacity, a spilled `SmallVec` points to its heap
// allocation, which stays in place when the `SmallVec` (and its allocator) is
// moved, as the `Allocator` contract requires. Otherwise it is empty or holds
// zero-sized items, which `LocatedLength` treats as inline, and the pointer is
// dangling like `Vec`'s, so it does not change either. With inline capacity
// this would not hold, since the inline elements move together with it.
unsafe impl<Item, Heap: Allocator> StableDeref for SmallVec<Item, 0, Heap> {}
