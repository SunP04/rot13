pub trait Cipher {
    type Output;

    fn encode(&self) -> Self::Output;
    fn decode(&self) -> Self::Output;
}
