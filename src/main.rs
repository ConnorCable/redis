mod serialize;

use tokio::net::TcpListener;
use std::io;


#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (_socket, _) = listener.accept().await?;
        println!("ping!");
    }


}

async fn process<T> (_socket: T) {
    println!("Ping!");
}
