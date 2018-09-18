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
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

const CONNECTION_STRING: &str = "127.0.0.1:4091";

fn main() {
    println!("わたしがクライアントだぜ☆（＾～＾） サーバーをテストするのに使えだぜ☆（＾～＾）");

    // サーバーに接続する。
    let mut stream = TcpStream::connect(CONNECTION_STRING).expect("Couldn't connect to the server...");
    println!("サーバーにはもう　つないである☆（＾～＾）");

    // ずっと。
    loop {
        println!("なんか入力しろだぜ☆（＾～＾） 終わるときは quit ☆（＾～＾）");

        // コマンド プロンプトからの入力があるまで待機します。
        //
        // 例。
        // LOGIN <username> <password>
        // 
        // LOGOUT
        //
        //
        // AGREE[ <GameID>]
        // （注：[]内は省略可能である事を示す。以下も同様。）
        //
        // REJECT[ <GameID>]
        //
        //
        // 指し手
        // +7776FU
        //
        // %TORYO
        //
        // %KACHI
        //
        // %CHUDAN
        //
        // またクライアントは対局中、手番にかかわらず、長さゼロの文字列、もしくはLF1文字のみをサーバに送信することができる。
        // クライアントは、これらの送信を頻繁に行ってはならない。 具体的には、当該クライアントからの何らかの文字列をサーバが受信してから30秒を経ずして同一のクライアントからこれらの送信を行ってはならない。
        // 対局中のクライアントの情報送信は、すべて自身の手番のときにのみ実行可能とする。
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("info Failed to read_line");

        if 3<line.len() && "quit" == &line[0..4] {
            break;
        }
        // FIXME マルチバイト文字の送信方法が分からん☆（＾～＾）
        let _ = stream.write(line.as_bytes());
        let _ = stream.flush();
    }
    /*
        let mut buf = vec![];
        match stream.read_to_end(&mut buf) {
            Ok(a) => {
                println!("Buf: {}.", a);
                break
            },
            Err(e) => panic!("encountered IO error: {}", e),
        };
    */

    println!("End client.");
}