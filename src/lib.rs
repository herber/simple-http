
#![deny(warnings)]

extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

pub fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        let res = "http/2.0 200 ok
        host: localhost
        content-type: text/html

        <h1>this is a test<h1/>\n";

        let (reader, writer) = socket.split();
        println!("socket={:?}", reader);

        let connection = io::write_all(writer, res)
            .then(|res| {
                println!("wrote message; success={:?}", res.is_ok());
                Ok(())
            });

        tokio::spawn(connection);


        Ok(())
    })
    .map_err(|err| {
        println!("accept error = {:?}", err);
    });

    println!("magic is going on at the {port}s", port=6142);
    tokio::run(server);
}
