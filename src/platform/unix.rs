use pin_project::pin_project;
use std::{
  io,
  pin::Pin,
  task::{Context, Poll},
};
use tokio::{
  io::{AsyncRead, AsyncWrite, ReadBuf},
  net::{UnixListener, UnixStream},
};
use uuid::{Uuid, fmt::Hyphenated};

const PATH_PREFIX: &str = "/tmp/aubypipe-";
const PATH_LEN: usize = PATH_PREFIX.len() + Hyphenated::LENGTH;

pub struct PipeListener {
  listener: UnixListener,
  path: heapless::String<PATH_LEN>,
}

impl PipeListener {
  pub fn pipe_name(&self) -> &str {
    self.path.as_str()
  }

  pub async fn accept(self) -> io::Result<PipeServer> {
    let (stream, _) = self.listener.accept().await?;

    Ok(PipeServer { stream })
  }
}

pub fn new_pipe() -> io::Result<PipeListener> {
  let mut path = heapless::String::new();
  use std::fmt::Write;
  write!(&mut path, "{PATH_PREFIX}{}", Uuid::new_v4().as_hyphenated()).unwrap();

  let listener = UnixListener::bind(path.as_str())?;

  Ok(PipeListener { listener, path })
}

#[pin_project]
pub struct PipeServer {
  #[pin]
  stream: UnixStream,
}

pub type ServerInnerStream = UnixStream;

impl PipeServer {
  pub fn as_inner(&mut self) -> &mut ServerInnerStream {
    &mut self.stream
  }
}

impl AsyncRead for PipeServer {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    UnixStream::poll_read(self.project().stream, cx, buf)
  }
}

impl AsyncWrite for PipeServer {
  fn poll_write(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, io::Error>> {
    UnixStream::poll_write(self.project().stream, cx, buf)
  }

  fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    UnixStream::poll_flush(self.project().stream, cx)
  }

  fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    UnixStream::poll_shutdown(self.project().stream, cx)
  }
}

pub async fn connect_to_pipe(pipe_name: &str) -> io::Result<PipeClient> {
  let stream = UnixStream::connect(pipe_name).await?;

  Ok(PipeClient { stream })
}

#[pin_project]
pub struct PipeClient {
  #[pin]
  stream: UnixStream,
}

pub type ClientInnerStream = UnixStream;

impl PipeClient {
  pub fn as_inner(&mut self) -> &mut ClientInnerStream {
    &mut self.stream
  }
}

impl AsyncRead for PipeClient {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    UnixStream::poll_read(self.project().stream, cx, buf)
  }
}

impl AsyncWrite for PipeClient {
  fn poll_write(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, io::Error>> {
    UnixStream::poll_write(self.project().stream, cx, buf)
  }

  fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    UnixStream::poll_flush(self.project().stream, cx)
  }

  fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    UnixStream::poll_shutdown(self.project().stream, cx)
  }
}
