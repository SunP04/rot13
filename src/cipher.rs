/// Trait that defines a basic Cipher
pub trait Cipher {
    /// Defines the output of the Cipher.
    /// It's recommended to return a concrete type to the user.
    type Output;

    /// Defines a stub function to encode a value.
    fn encode(&self) -> Self::Output;
    /// Defines a stub function to decode a value.
    fn decode(&self) -> Self::Output;
}
