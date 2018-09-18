/// # 参考URL。
/// - https://doc.rust-lang.org/std/net/struct.TcpStream.html |Struct std::net::TcpStream
/// 
/// # コマンド例。
/// 
/// ```
/// ### コンパイル(開発中)。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shogi_client
/// cargo clippy
/// 
/// ### コンパイル(リリース用)。
/// cargo build --release
/// 
/// ### 実行。
/// cargo run --release
/// ```
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

const CONNECTION_STRING: &str = "127.0.0.1:4091";

fn main() {
    println!("I am a client!");

    // 入力を受け取る。
    let mut stream = TcpStream::connect(CONNECTION_STRING).expect("Couldn't connect to the server...");

    let _ = stream.write(b"Hello, server!");
    /*
    // ずっと。
    loop {
        let mut buf = vec![];
        match stream.read_to_end(&mut buf) {
            Ok(a) => {
                println!("Buf: {}.", a);
                break
            },
            Err(e) => panic!("encountered IO error: {}", e),
        };
    }
    */

    println!("End client.");
}