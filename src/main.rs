use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

/// 处理客户端发来的请求
fn handle_connection(mut stream: TcpStream) {
    // 缓存，用于存放客户端发来的数据
    let mut buffer = [0; 1024];

    // 读取 TCP 流中的数据，并放到缓存中
    while match stream.read(&mut buffer) {
        Ok(size) => {
            // 打印客户端发来的数据
            println!("Received message: {}", String::from_utf8_lossy(&buffer[..]));
            // 给客户端返回刚收到的数据
            stream.write(&buffer[0..size]).unwrap();
            // 返回 true
            true
        }
        Err(_) => {
            // 打印错误信息
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            // 关闭 TCP 流
            stream.shutdown(Shutdown::Both).unwrap();
            // 返回 false
            false
        }
    } {}
}
fn main() {
    // 监听本机的 8000 端口
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    // 打印提示信息
    println!("Server is listening on port 8000 at localhost");

    // 接收并处理客户端发来的请求
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 打印客户端地址
                println!("New connection: {}", stream.peer_addr().unwrap());
                // 新建线程处理请求
                thread::spawn(move || {
                    // 调用处理函数
                    handle_connection(stream)
                });
            }
            Err(e) => {
                // 打印报错信息
                println!("Error: {:?}", e);
            }
        }
    }

    // 关闭监听
    drop(listener);
}
