#![doc = include_str!("../readme.md")]

use std::{
  io,
  pin::Pin,
  task::{Context, Poll},
};

use pin_project::pin_project;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::platform::{ClientInnerStream, ServerInnerStream};

mod platform {
  #[cfg(unix)]
  mod unix;
  #[cfg(windows)]
  mod windows;
  #[cfg(unix)]
  pub use unix::*;
  #[cfg(windows)]
  pub use windows::*;
}

/// Creates a [PipeListener]. The pipe name of the listener is chosen randomly.
/// See the crate-level documentation for an example of how to use this.
pub fn new_pipe() -> io::Result<PipeListener> {
  platform::new_pipe().map(PipeListener)
}

/// Listens for the client connection. Make sure to send the [pipe name](PipeListener::pipe_name)
/// to the client process before waiting with [accept](PipeListener::accept).
pub struct PipeListener(platform::PipeListener);

impl PipeListener {
  /// The name of the pipe. The string must be sent to the client process
  /// in order to establish a connection.
  pub fn pipe_name(&self) -> &str {
    self.0.pipe_name()
  }

  /// Waits for the client to connect.
  pub async fn accept(self) -> io::Result<PipeServer> {
    self.0.accept().await.map(PipeServer)
  }
}

/// A bi-directional pipe to the client process.
#[pin_project]
pub struct PipeServer(#[pin] platform::PipeServer);

impl PipeServer {
  /// Returns a reference to the internal pipe. On unix platforms, this is a
  /// UnixStream. On Windows, it is a NamedPipeServer.
  pub fn as_inner(&mut self) -> &mut ServerInnerStream {
    self.0.as_inner()
  }
}

impl AsyncRead for PipeServer {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    self.project().0.poll_read(cx, buf)
  }
}

impl AsyncWrite for PipeServer {
  fn poll_write(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, io::Error>> {
    self.project().0.poll_write(cx, buf)
  }

  fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    self.project().0.poll_flush(cx)
  }

  fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    self.project().0.poll_shutdown(cx)
  }
}

/// Connects to the named pipe with the provided name.
/// See the crate-level documentation for an example of how to use this.
pub async fn connect_to_pipe(pipe_name: &str) -> io::Result<PipeClient> {
  platform::connect_to_pipe(pipe_name).await.map(PipeClient)
}

/// A bi-directional pipe to the server process.
#[pin_project]
pub struct PipeClient(#[pin] platform::PipeClient);

impl PipeClient {
  /// Returns a reference to the internal pipe. On Unix platforms, this is a
  /// UnixStream. On Windows, it is a NamedPipeClient.
  pub fn as_inner(&mut self) -> &mut ClientInnerStream {
    self.0.as_inner()
  }
}

impl AsyncRead for PipeClient {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    self.project().0.poll_read(cx, buf)
  }
}

impl AsyncWrite for PipeClient {
  fn poll_write(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, io::Error>> {
    self.project().0.poll_write(cx, buf)
  }

  fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    self.project().0.poll_flush(cx)
  }

  fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    self.project().0.poll_shutdown(cx)
  }
}
