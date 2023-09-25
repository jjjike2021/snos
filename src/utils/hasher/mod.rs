use cairo_felt::Felt252;

pub mod pedersen;

/// A trait for hashing.
pub trait HasherT {
    /// Hashes the given data.
    /// The hash is computed by first converting the data to bytes and then hashing the bytes.
    /// It is handled dynamically by hashing in chunks of 31 bytes.
    /// # Arguments
    /// * `data` - The data to hash.
    /// # Returns
    /// The hash of the data.
    fn hash_bytes(data: &[u8]) -> Felt252;

    /// Hashes the 2 felts sent.
    ///
    /// # Arguments
    ///
    /// * `a` - First element to hash.
    /// * `b` - Second element to hash.
    ///
    /// # Returns
    ///
    /// The hash of the 2 values.
    fn hash_elements(a: Felt252, b: Felt252) -> Felt252;

    /// Computes a hash chain over the data, in the following order:
    /// h(h(h(h(0, data\[0\]), data\[1\]), ...), data\[n-1\]), n).
    /// The hash is initialized with 0 and ends with the data length appended.
    /// The length is appended in order to avoid collisions of the following kind:
    /// H(\[x,y,z\]) = h(h(x,y),z) = H(\[w, z\]) where w = h(x,y).
    ///
    /// # Arguments
    ///
    /// * `elements` - The array to hash.
    ///
    /// # Returns
    ///
    /// The hash of the array.
    fn compute_hash_on_elements(elements: &[Felt252]) -> Felt252;
}
