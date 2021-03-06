//! Implementations of the [`Connection`] trait for various built-in types
// TODO: impl Connection for all `Read + Write` (blocked on specialization)

#[cfg(feature = "alloc")]
mod boxed;

#[cfg(feature = "std")]
mod tcpstream;

#[cfg(all(feature = "std", unix))]
mod unixstream;

use super::Connection;

impl<E> Connection for &mut dyn Connection<Error = E> {
    type Error = E;

    fn read(&mut self) -> Result<u8, Self::Error> {
        (**self).read()
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        (**self).read_exact(buf)
    }

    fn write(&mut self, byte: u8) -> Result<(), Self::Error> {
        (**self).write(byte)
    }

    fn peek(&mut self) -> Result<Option<u8>, Self::Error> {
        (**self).peek()
    }
}
