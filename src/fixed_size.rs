/// Trait for types that can be encoded with a known, fixed size.
pub trait FixedSize {
    /// The size of the encoded data if Self has a fixed encoding size
    fn encoded_size() -> usize;
}
