use std::io::prelude::*;
use std::io::ErrorKind::Interrupted;
use std::io::Result;

/// Adds the [`fill`](Fill::fill) method to Read implementors.
pub trait Fill: Read {
    /// Fill the given buffer. This will call `read` on `self` until `read`
    /// returns `0` or an error which is not [`ErrorKind::Interrupted`](std::io::ErrorKind::Interrupted),
    /// indicating that there is no more data available currently, or the buffer `buf`
    /// is full. See also [`read_to_end`](std::io::Read::read_to_end), which operates similarly.
    ///
    /// ```
    /// # use std::io::{Cursor, Error};
    /// use fill::Fill;
    /// let mut cursor = Cursor::new("Hello, World!");
    /// let mut buf = [0_u8; 20];
    /// assert_eq!(cursor.fill(&mut buf)?, 13);
    /// # Ok::<(), Error>(())
    /// ```
    fn fill(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut bytes_read: usize = 0;
        loop {
            match self.read(&mut buf[bytes_read..]) {
                Err(e) => match e.kind() {
                    Interrupted => {
                        continue;
                    }
                    _ => return Err(e),
                },
                Ok(0) => return Ok(bytes_read),
                Ok(l) => bytes_read += l,
            };
    }}
}

/// Implement `Fill` for all types that implement [`Read`].
impl<R: Read> Fill for R {}
 
/// An [`Iterator`] which wraps a [`Read`]er. Each call to [`next`](ChunkedReader::next) returns
/// the next chunk of `self.size` as a `Result<Vec<u8>>`.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct ChunkedReader<R: Read> {
    read: R,
    size: usize,
}

impl<R: Read> ChunkedReader<R> {
    /// Consumes the [`ChunkedReader`], returning the underlying [`Read`]er.
    pub fn into_inner(self) -> R {
        self.read
    }
}

impl<R: Read> Iterator for ChunkedReader<R> {
    type Item = Result<Vec<u8>>;

    /// Each call attempts to [`fill`](Fill::fill) a `Vec<u8>` of `self.size`. If an Error is thrown, then the
    /// returned result contains the error. If `fill` returns `0` bytes, then `None` is returned.
    /// If `fill` returns `n` bytes, then a `Vec<u8>` of size `n` is returned.  
    fn next(&mut self) -> Option<Result<Vec<u8>>> {
        let mut buf = vec![0_u8; self.size];
        match self.read.fill(&mut buf) {
            Err(e) => Some(Err(e)),
            Ok(0) => None,
            Ok(l) => {
                buf.truncate(l);
                Some(Ok(buf))
            }
        }
    }
}

trait Chunk: Read {
    /// Consumes `self`, returning a [`ChunkedReader`] over `self`.
    ///
    /// Panics if `size` is `0`.
    fn chunked(self, size: usize) -> ChunkedReader<Self>
    where
        Self: Sized,
    {
        assert!(size != 0);
        ChunkedReader { read: self, size }
    }
}

/// Implement `Chunk` for all types that implement [`Read`].
impl<R: Read> Chunk for R {}

