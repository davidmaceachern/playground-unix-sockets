use neon::prelude::*;
use bytes::BytesMut;
use serde_json::json;
use std::io::prelude::*;
use std::io::{self, Error, ErrorKind};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::str::from_utf8;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut stream = match UnixStream::connect("/home/davidmaceachern/github/nodesozu/tmp/sock") {
        Err(_) => panic!("Failed to connect to socket"),
        Ok(_) => Ok(stream),
    };
    
    let CommandRequest = json!({
    "id": "ID_CHAT",
    "version": 0,
    "type": "PROXY",
    "data": {
        "type": "STATUS"
    }});

    let mut buf = BytesMut::with_capacity(1024);
    let message = serde_json::to_string(&CommandRequest);

    buf.extend(message.as_bytes());
    buf.extend(&[0u8][..]);


    match stream.write_all(&buf) {
        Err(_) => panic!("couldn't send message"),
        Ok(_) => println!("Sent"),
    }

    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});


