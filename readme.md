# aubypipe
A simple IPC wrapper for Tokio's Unix Domain Sockets and Windows named pipe.

```rust
use aubypipe::{new_pipe, connect_to_pipe, PipeListener, PipeServer, PipeClient};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  // start out by creating a pipe
  let listener: PipeListener = new_pipe()?;
  let pipe_name: String = listener.pipe_name().to_owned();
  
  let client_task = tokio::spawn(async move {
    // connect after receiving the pipe name
    let mut client: PipeClient = connect_to_pipe(&pipe_name).await.unwrap();

    // we can receive bytes from the server
    let mut buffer: Vec<u8> = Vec::with_capacity(32);
    client.read_buf(&mut buffer).await.unwrap();
    assert_eq!("hello world", str::from_utf8(&buffer).unwrap());

    // and send bytes back too!
    client.write_all(b"goodbye").await.unwrap();
  });

  // once we've sent out the pipe name, we wait for the client to connect
  let mut server: PipeServer = listener.accept().await?;

  // now that we're connected, we can send bytes to the client
  server.write_all(b"hello world").await.unwrap();

  // and the client can send bytes too
  let mut buffer: Vec<u8> = Vec::with_capacity(32);
  server.read_buf(&mut buffer).await.unwrap();
  assert_eq!("goodbye", str::from_utf8(&buffer).unwrap());

  client_task.await.unwrap();
  Ok(())
}
```
