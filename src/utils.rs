//! Utilities used by various things.

/// An iterator that treats `slice` as a ring buffer and iterates
/// from a starting index. Created using the `ring` function below.
pub struct RingIterator<'a, T> {
    slice: &'a [T],
    index: usize,
    len: usize,
    visited: usize,
}

impl<'a, T> Iterator for RingIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.visited == self.len {
            None
        } else {
            let index = self.index;
            self.index = (self.index + 1) % self.len;
            self.visited += 1;
            Some(&self.slice[index])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len - self.visited, Some(self.len - self.visited))
    }
}

/// Create a ring iterator starting from `index`.
/// 
/// # Examples
/// 
/// ```
/// let it =  ring(&[1, 2, 3], 1);
/// assert_eq!(it.next(), 2);
/// ```
pub fn ring<'a, T>(slice: &'a [T], index: usize) -> RingIterator<'a, T> {
    RingIterator {
        slice,
        index,
        len: slice.len(),
        visited: 0,
    }
}
