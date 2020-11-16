use bytes::BytesMut;
use serde_json::json;
use std::io::prelude::*;
use std::io::{self, Error, ErrorKind};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::str::from_utf8;
fn main() -> std::io::Result<()> {

    let mut stream = UnixStream::connect("/home/davidmaceachern/github/nodesozu/tmp/sock")?;
    //    stream.set_nonblocking(true).expect("Couldn't set nonblocking");
    let CommandRequest = json!({
    "id": "ID_CHAT",
    "version": 0,
    "type": "PROXY",
    "data": {
        "type": "STATUS"
    }});
    let mut buf = BytesMut::with_capacity(1024);
    let message = serde_json::to_string(&CommandRequest)?;
    println!("encoded message: {:?}", message);
    buf.extend(message.as_bytes());
    //println!("{:?}", buf);
    buf.extend(&[0u8][..]);
    //println!("buffercontent: {:?}", from_utf8(&buf[..]));
    // Send message
    match stream.write_all(&buf) {
        Err(_) => panic!("couldn't send message"),
        Ok(_) => println!("Sent"),
    }
    // TODO: create listener to receive this message? 
    let mut response = String::new();
    let string = stream.read_to_string(&mut response)?;
    println!("We received this as a response {}", string);
    //    match stream.write_all(mut buf: &[u8]){
    //        Err(_) => panic!("couldn't send message"),
    //        Ok(_) => {}
    //    }
    //    let mut response = String::new();
    //    println!("Receiving");
    //    stream.read_to_string(&mut response)?;
    //    println!("Printing");
    //    println!("{}", response);
    //    stream
    //        .shutdown(Shutdown::Both)
    //        .expect("shutdown function failed");
    Ok(())
}
