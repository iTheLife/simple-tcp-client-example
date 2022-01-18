use std::{sync::Arc, time::Duration};

use my_tcp_sockets::TcpClient;

use crate::my_client::{MyCallback, MySerializer};
mod my_client;

#[tokio::main]
async fn main() {
    let client = TcpClient::new("my-example".to_string(), "127.0.0.1:8080".to_string());

    let serial = Arc::new(|| -> MySerializer { MySerializer::new() });
    let callback = Arc::new(MyCallback::new());
    client.start(serial, callback);
    println!("Hello, world!");

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

// use tokio::{
//     io::AsyncReadExt,
//     net::{TcpListener, TcpStream},
// };

// use std::{io, time::Duration};

// async fn process(mut socket: TcpStream) {
//     println!("Privet world");
//     let mut buffer = String::new();

//     match socket.read_i16() {}

//     println!("{:?}",);

//     tokio::time::sleep(Duration::from_secs(10)).await;
// }

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

//     loop {
//         let (socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             // Process each socket concurrently.
//             process(socket).await
//         });
//     }
// }
