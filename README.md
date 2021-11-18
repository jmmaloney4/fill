# fill

A crate which implements a [`fill`](Fill::fill) method for [`Read`]ers and uses it to implement a
[`ChunkedReader`](ChunkedReader) which wraps a reader in an `Iterator` of `Vec<u8>`.
