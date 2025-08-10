use pin_project::pin_project;
use std::{
  io,
  pin::Pin,
  task::{Context, Poll},
};
use tokio::{
  io::{AsyncRead, AsyncWrite, ReadBuf},
  net::windows::named_pipe::{ClientOptions, NamedPipeClient, NamedPipeServer, ServerOptions},
};
use uuid::{Uuid, fmt::Hyphenated};

const PATH_PREFIX: &str = r"\\.\pipe\aubypipe-";
const PATH_LEN: usize = PATH_PREFIX.len() + Hyphenated::LENGTH;

pub struct PipeListener {
  server: NamedPipeServer,
  path: heapless::String<PATH_LEN>,
}

impl PipeListener {
  pub fn pipe_name(&self) -> &str {
    self.path.as_str()
  }

  pub async fn accept(self) -> io::Result<PipeServer> {
    self.server.connect().await?;

    Ok(PipeServer {
      server: self.server,
    })
  }
}

pub fn new_pipe() -> io::Result<PipeListener> {
  let mut path = heapless::String::new();
  use std::fmt::Write;
  write!(&mut path, "{PATH_PREFIX}{}", Uuid::new_v4().as_hyphenated()).unwrap();

  let server = ServerOptions::new().create(path.as_str())?;

  Ok(PipeListener { server, path })
}

#[pin_project]
pub struct PipeServer {
  #[pin]
  server: NamedPipeServer,
}

pub type ServerInnerStream = NamedPipeServer;

impl PipeServer {
  pub fn as_inner(&mut self) -> &mut ServerInnerStream {
    &mut self.server
  }
}

impl AsyncRead for PipeServer {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    NamedPipeServer::poll_read(self.project().server, cx, buf)
  }
}

impl AsyncWrite for PipeServer {
  fn poll_write(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, io::Error>> {
    NamedPipeServer::poll_write(self.project().server, cx, buf)
  }

  fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    NamedPipeServer::poll_flush(self.project().server, cx)
  }

  fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    NamedPipeServer::poll_shutdown(self.project().server, cx)
  }
}

pub async fn connect_to_pipe(pipe_name: &str) -> io::Result<PipeClient> {
  let client = ClientOptions::new().open(pipe_name)?;

  Ok(PipeClient { client })
}

#[pin_project]
pub struct PipeClient {
  #[pin]
  client: NamedPipeClient,
}

pub type ClientInnerStream = NamedPipeClient;

impl PipeClient {
  pub fn as_inner(&mut self) -> &mut ClientInnerStream {
    &mut self.client
  }
}

impl AsyncRead for PipeClient {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    self.project().client.poll_read(cx, buf)
  }
}

impl AsyncWrite for PipeClient {
  fn poll_write(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, io::Error>> {
    NamedPipeClient::poll_write(self.project().client, cx, buf)
  }

  fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    NamedPipeClient::poll_flush(self.project().client, cx)
  }

  fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
    NamedPipeClient::poll_shutdown(self.project().client, cx)
  }
}
