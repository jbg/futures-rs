use crate::io::AsyncRead;
use futures_core::future::Future;
use futures_core::task::{Context, Poll};
use std::io;
use std::marker::Unpin;
use std::mem::PinMut;

/// A future which can be used to easily read available number of bytes to fill
/// a buffer.
///
/// Created by the [`read`] function.
#[derive(Debug)]
pub struct Read<'a, R: ?Sized + 'a> {
    rd: &'a mut R,
    buf: &'a mut [u8],
}

// Pinning is never projected to fields
impl<'a, R: ?Sized> Unpin for Read<'a, R> {}

pub fn read<'a, R>(rd: &'a mut R, buf: &'a mut [u8]) -> Read<'a, R>
    where R: AsyncRead + ?Sized,
{
    Read { rd, buf }
}

impl<'a, R> Future for Read<'a, R>
    where R: AsyncRead + ?Sized,
{
    type Output = io::Result<usize>;

    fn poll(mut self: PinMut<Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = &mut *self;
        this.rd.poll_read(cx, this.buf)
    }
}
