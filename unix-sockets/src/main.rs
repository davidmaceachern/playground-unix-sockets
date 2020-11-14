use serde_json::json;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/home/davidmaceachern/github/nodesozu/tmp/sock")?;
    // TODO: identify how to send this message
    let request = json!({
    "id": "ID_TEST",
    "version": 0,
    "data": {
      "type": "STATUS",
    },
    "worker_id": 0
      });

    let packaged = request.to_string();
    stream.write_all(packaged.as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);
    Ok(())
}
